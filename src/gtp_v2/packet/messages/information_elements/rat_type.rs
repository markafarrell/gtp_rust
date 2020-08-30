use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
pub enum RATType {
    Reserved = 0,
    UTRAN = 1,
    GERAN = 2,
    WLAN = 3,
    GAN = 4,
    HspaEvolution = 5,
    EUTRAN = 6,
    Virtual = 7,
    EutranNbIot = 8,
    LteM = 9,
    NR = 10,
}

impl TryFrom<u8> for RATType
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(RATType::Reserved),
            1 => Ok(RATType::UTRAN),
            2 => Ok(RATType::GERAN),
            3 => Ok(RATType::WLAN),
            4 => Ok(RATType::GAN),
            5 => Ok(RATType::HspaEvolution),
            6 => Ok(RATType::EUTRAN),
            7 => Ok(RATType::Virtual),
            8 => Ok(RATType::EutranNbIot),
            9 => Ok(RATType::LteM),
            10 => Ok(RATType::NR),
            _ => Err(format!("Unsupported RAT type ({})", value))
        }
    }
}

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (82)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       | RAT Type                                                      |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub rat_type: RATType
}

impl InformationElement {
    pub fn new(rat_type: RATType, instance: u8) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    rat_type: rat_type,
                    instance: instance,
                }
            )
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

        if let Ok(rat_type) = RATType::try_from(buffer[pos]){
            // pos = pos + 1;

            Some(
                (
                    InformationElement {
                        rat_type: rat_type,
                        instance,
                    },
                    (length + 4) as usize
                )
            )
        }
        else {
            None
        }
        
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::RATType
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

        4+1
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

        buffer[pos] = self.rat_type as u8;
        pos = pos + 1;

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

        let ie = InformationElement::new(RATType::EUTRAN, 0).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::RATType as u8,
            0, 1, // Length
            0, // Spare
            6 // RAT Type
        ]);
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(RATType::EUTRAN, 0).unwrap();
        assert_eq!(ie.length(), 5);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(RATType::EUTRAN, 0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::RATType as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::RATType as u8,
            0, 1, // Length
            0, // Spare
            6 // RAT Type
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.rat_type as u8, RATType::EUTRAN as u8);
        }
        else {
            assert!(false);
        }
    }
}