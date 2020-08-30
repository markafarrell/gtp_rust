use super::{InformationElementTraits, InformationElementType, LENGTH};

use std::convert::TryInto;

use byteorder::{ByteOrder, NetworkEndian};

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (1)                                                   |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        2       | MSISDN[1]                     | MSISDN[0]                     |
        3       | MSISDN[3]                     | MSISDN[2]                     |
        4 -> n  | MSISDN[m]                     | MSISDN[m-1]                 |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub msisdn: Vec<u8>,
}

impl InformationElement {
    pub fn new(msisdn: &str, instance: u8) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            let mut parsed_msisdn:Vec<u8> = Vec::new();

            for c in msisdn.chars() {
                let digit = c.to_string().parse::<u8>();
    
                let digit = match digit {
                    Ok(n) => n,
                    Err(_e) => {
                        return Err("Could not parse MSISDN".to_string())
                    }
                };
    
                parsed_msisdn.push(digit);
            }
    
            Ok(InformationElement {
                msisdn: parsed_msisdn,
                instance
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

        let mut msisdn: Vec<u8> = Vec::new();

        while pos < (length+4) as usize {
            let first_digit = buffer[pos] & 0xF;
            let second_digit = (buffer[pos] >> 4) & 0xF;

            msisdn.push(first_digit);

            if second_digit != 0xF {
                msisdn.push(second_digit);
            }

            pos = pos + 1;
        }

        Some(
            (
                InformationElement {
                    msisdn,
                    instance,
                },
                pos
            )
        )
    }

    pub fn generate_tbcd_msisdn(&self) -> Vec<u8> {
        let mut tbcd_msisdn: Vec<u8> = Vec::new();

        let mut msisdn_index = 0;

        while msisdn_index < self.msisdn.len() {
            let mut tbcd_digit = self.msisdn[msisdn_index] & 0xF;
            msisdn_index = msisdn_index + 1;

            if msisdn_index < self.msisdn.len() {
                tbcd_digit = tbcd_digit | ((self.msisdn[msisdn_index] & 0xF) << 4);
                msisdn_index = msisdn_index + 1;
            }
            else {
                // Top nibble of odd length MSISDN should be 0xF
                tbcd_digit = tbcd_digit | 0xF0;
            }
            tbcd_msisdn.push(tbcd_digit);
        }

        tbcd_msisdn
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::MSISDN
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
        (4 + (self.msisdn.len() / 2) + (self.msisdn.len() % 2)).try_into().unwrap()
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

        let tbcd_msisdn = self.generate_tbcd_msisdn();

        for i in 0..tbcd_msisdn.len() {
            buffer[pos] = tbcd_msisdn[i];
            pos = pos + 1;
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
    fn test_new()
    {
        let msisdn_valid = "61444555666";
        let msisdn_ie = InformationElement::new(msisdn_valid, 0);

        match msisdn_ie {
            Ok(_) => {
                assert_eq!(true, true);
            }
            Err(e) => {
                println!("{}", e);
                assert_eq!(false, true)
            }
        }

        let msisdn_invalid = "6f444555666";
        let msisdn_ie = InformationElement::new(msisdn_invalid, 0);

        match msisdn_ie {
            Ok(_) => {
                assert_eq!(false, true)
            }
            Err(e) => {
                assert_eq!(e, "Could not parse MSISDN".to_string())
            }
        }
    }

    #[test]
    fn test_generate_tbcd_msisdn()
    {
        let msisdn = "61123456789";
        let msisdn_ie = InformationElement::new(msisdn, 0);

        assert_eq!(msisdn_ie.is_ok(), true);

        if let Ok(i) = msisdn_ie {
            assert_eq!(i.generate_tbcd_msisdn(), [0x16, 0x21, 0x43, 0x65, 0x87, 0xF9]);
        }
        else {
            assert_eq!(false, true)
        }
        
        let msisdn = "611234567890";
        let msisdn_ie = InformationElement::new(msisdn, 0);

        assert_eq!(msisdn_ie.is_ok(), true);

        if let Ok(i) = msisdn_ie {
            assert_eq!(i.generate_tbcd_msisdn(), [0x16, 0x21, 0x43, 0x65, 0x87, 0x09]);
        }
        else {
            assert_eq!(false, true)
        }  
    }

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let msisdn = "61123456789";
        let msisdn_ie = InformationElement::new(msisdn, 0);

        match msisdn_ie {
            Ok(i) => {
                let pos = i.generate(&mut buffer);
                assert_eq!(buffer[..pos], [
                    InformationElementType::MSISDN as u8,
                    0, 6, // Length
                    0, // Spare
                    0x16, 0x21, 0x43, 0x65, 0x87, 0xF9]);
            }
            Err(e) => {
                println!("{}", e);
                assert_eq!(false, true)
            }
        }
    }
    
    #[test]
    fn test_length() {
        let msisdn = "61123456789";
        let ie = InformationElement::new(msisdn, 0);

        if let Ok(i) = ie {
            assert_eq!(i.length(), 6+4)
        }
        else {
            assert_eq!(false, true)
        }  
        let msisdn = "611234567890";
        let ie = InformationElement::new(msisdn, 0);

        if let Ok(i) = ie {
            assert_eq!(i.length(), 6+4)
        }
        else {
            assert_eq!(false, true)
        }    
    }

    #[test]
    fn test_message_type() {
        let msisdn = "61123456789";
        let ie = InformationElement::new(msisdn, 0);

        if let Ok(i) = ie {
            assert_eq!(i.information_element_type() as u8, InformationElementType::MSISDN as u8)
        }
        else {
            assert_eq!(false, true);
        }
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [
            InformationElementType::MSISDN as u8,
            0, 6, // Length
            0, // Spare
            0x16, 0x21, 0x43, 0x65, 0x87, 0xF9,
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.msisdn, [6, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        }
        else {
            assert!(false);
        }

        let ie_bytes = [
            InformationElementType::MSISDN as u8,
            0, 6, // Length
            0, // Spare
            0x16, 0x21, 0x43, 0x65, 0x87, 0x09,
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.msisdn, [6, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
        }
        else {
            assert!(false);
        }
    }
}