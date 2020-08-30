use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

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
        5       | Charging Characteristics (Octet 1)                            |
        6       | Charging Characteristics (Octet 2)                            |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub charging_characteristics: u16
}

impl InformationElement {
    pub fn new(charging_characteristics: u16, instance: u8) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    charging_characteristics,
                    instance,
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

        let charging_characteristics = NetworkEndian::read_u16(&buffer[pos..pos+2]);
        // pos = pos + 2;
    
        Some(
            (
                InformationElement {
                    charging_characteristics,
                    instance,
                },
                (length + 4) as usize
            )
        )
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::ChargingCharacteristics
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

        4+2
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

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.charging_characteristics);
        pos = pos + 2;

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

        let ie = InformationElement::new(0x1234, 0).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::ChargingCharacteristics as u8,
            0, 2, // Length
            0, // Spare
            0x12, 0x34 // Charging Characteristics
        ]);
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(0x1234, 0).unwrap();
        assert_eq!(ie.length(), 6);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(0x1234, 0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::ChargingCharacteristics as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::ChargingCharacteristics as u8,
            0, 2, // Length
            0, // Spare
            0x12, 0x34 // Charging Characteristics
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.charging_characteristics, 0x1234);
        }
        else {
            assert!(false);
        }
    }
}