use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (73)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       | Spare                         | EPS Bearer ID                 |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub eps_bearer_id: u8
}

impl InformationElement {
    pub fn new(eps_bearer_id: u8, instance: u8) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else if eps_bearer_id > 0xF {
            Err(format!("EPS Bearer ID is > 0xF {}", eps_bearer_id))
        }
        else {
            Ok(InformationElement {
                eps_bearer_id: eps_bearer_id,
                instance: instance,
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

        let eps_bearer_id = buffer[pos] & 0xF;
        // pos = pos + 1;

        Some(
            (
                InformationElement {
                    eps_bearer_id: eps_bearer_id,
                    instance,
                },
                (length + 4) as usize
            )
        )        
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::EBI
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

        buffer[pos] = self.eps_bearer_id as u8;
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

        if let Ok(ie) = InformationElement::new(0xF, 0) {
            let pos = ie.generate(&mut buffer);

            assert_eq!(buffer[..pos], [InformationElementType::EBI as u8,
                0, 1, // Length
                0, // Spare
                0xF // EPS Bearer ID
            ]);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = InformationElement::new(0xFF, 0) {
            // This should fail EPS Bearer ID must be less than 0xF
            assert!(false);
        }
        else {
            assert!(true);
        }
    }

    #[test]
    fn test_length() {
        if let Ok(ie) = InformationElement::new(0x2, 0){
            assert_eq!(ie.length(), 5);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn test_message_type() {
        if let Ok(ie) = InformationElement::new(0x2, 0){
            assert_eq!(ie.information_element_type() as u8, InformationElementType::EBI as u8);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::RATType as u8,
            0, 1, // Length
            0, // Spare
            0x5 // EPS Bearer ID
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.eps_bearer_id, 0x5);
        }
        else {
            assert!(false);
        }
    }
}