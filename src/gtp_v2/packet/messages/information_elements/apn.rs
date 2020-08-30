extern crate ascii;

use byteorder::{ByteOrder, NetworkEndian};

use std::convert::TryInto;
use ascii::{AsciiString, AsciiChar, ToAsciiChar};

use super::{InformationElementTraits, InformationElementType, LENGTH};

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (3)                                                   |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5 -> n  | Access Point Name                                             |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub apn: AsciiString
}

impl InformationElement {
    pub fn new(apn: AsciiString, instance: u8) -> Result<Self, String> {
        // NOTE: APN string must be < 100 octets long and only contain ASCII characters

        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(InformationElement {
                apn: apn,
                instance: 0,
            })
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;
        
        // Read the type
        let _ie_type = buffer[pos];
        pos = pos + 1;

        // Read the length
        let length = NetworkEndian::read_u16(&buffer[LENGTH]);
        pos = pos + 2;

        //Spare and instance
        let instance = buffer[pos] & 0xF;
        pos = pos + 1;

        let mut apn = AsciiString::new();

        while pos < (length+4) as usize {
            // Each label is encoded with the lenght first then label

            let label_length = buffer[pos];
            pos = pos + 1;

            for _ in 0..label_length {

                if let Ok(ch) = buffer[pos].to_ascii_char() {
                    apn.push(ch)
                }
                pos = pos + 1;
            }
            apn.push(AsciiChar::Dot);
        }

        apn.pop(); // Pop off the last dot

        Some(
            (
                InformationElement {
                    apn,
                    instance,
                },
                pos
            )
        )
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::APN
    }

    fn instance(&self) -> u8 {
        self.instance
    }

    fn set_instance(&mut self, instance: u8) -> Result<u8, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            self.instance = instance;
            Ok(self.instance)
        }
    }

    fn length(&self) -> u16 {
        /* This is the actual length of the Information element INCLUDING the first 4 octets
        To calculate the length field of the IE you need to subtract 4 from what is returned */

        ((4 + self.apn.len()+1) as usize).try_into().unwrap()
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = 0;
        
        // Write the type
        buffer[pos] = self.information_element_type() as u8;
        pos = pos + 1;

        // Write the length
        // We subtract 4 octets as the type and length fields aren't included.
        NetworkEndian::write_u16(&mut buffer[LENGTH], self.length()-4);
        pos = pos + 2;

        //Spare and instance
        buffer[pos] = self.instance & 0xF;
        pos = pos + 1;

        // Split the apn into labels

        for l in self.apn.split(AsciiChar::Dot) {
            let length = l.len();

            buffer[pos] = (length as u8).try_into().unwrap();
            pos = pos + 1;

            for i in 0..length {
                buffer[pos] = l[i] as u8;
                pos = pos + 1;
            }
        }

        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    use crate::gtp_v2::packet::messages::information_elements::InformationElementType;

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let ie = InformationElement::new(AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap(), 0).unwrap();

        let pos = ie.generate(&mut buffer);

        let expected = [InformationElementType::APN as u8,
            0, 31, // Length
            0, // Spare
            7, AsciiChar::a as u8, AsciiChar::w as u8,  AsciiChar::e as u8, AsciiChar::s as u8, AsciiChar::o as u8, AsciiChar::m as u8,  AsciiChar::e as u8, 
            3, AsciiChar::a as u8, AsciiChar::p as u8, AsciiChar::n as u8,
            6, AsciiChar::m as u8, AsciiChar::n as u8, AsciiChar::c as u8, AsciiChar::_0 as u8,  AsciiChar::_9 as u8, AsciiChar::_9 as u8, 
            6, AsciiChar::m as u8, AsciiChar::c as u8, AsciiChar::c as u8, AsciiChar::_5 as u8,  AsciiChar::_0 as u8, AsciiChar::_5 as u8, 
            4, AsciiChar::g as u8, AsciiChar::p as u8,  AsciiChar::r as u8, AsciiChar::s as u8, 
        ];

        for i in 0..pos {
            if buffer[i] != expected[i] {
                println!("{} (actual) != {} (expected) at byte {}", buffer[i], expected[i], i);
                assert!(false);
            } 
        }
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap(), 0).unwrap();
        assert_eq!(ie.length(), 31+4);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap(), 0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::APN as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::APN as u8,
            0, 31, // Length
            0, // Spare
            7, AsciiChar::a as u8, AsciiChar::w as u8,  AsciiChar::e as u8, AsciiChar::s as u8, AsciiChar::o as u8, AsciiChar::m as u8,  AsciiChar::e as u8, 
            3, AsciiChar::a as u8, AsciiChar::p as u8, AsciiChar::n as u8,
            6, AsciiChar::m as u8, AsciiChar::n as u8, AsciiChar::c as u8, AsciiChar::_0 as u8,  AsciiChar::_9 as u8, AsciiChar::_9 as u8, 
            6, AsciiChar::m as u8, AsciiChar::c as u8, AsciiChar::c as u8, AsciiChar::_5 as u8,  AsciiChar::_0 as u8, AsciiChar::_5 as u8, 
            4, AsciiChar::g as u8, AsciiChar::p as u8,  AsciiChar::r as u8, AsciiChar::s as u8, 
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.apn, AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap());
        }
        else {
            assert!(false);
        }
    }
}