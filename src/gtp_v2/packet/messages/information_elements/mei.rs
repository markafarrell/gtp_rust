use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (75)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       | RAT Type                                                      |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub imei: [u8; 15],
    pub sv: Option<u8>,
}

impl InformationElement {
    pub fn new(imei: [u8; 15], sv: Option<u8>, instance: u8) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    imei,
                    sv,
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

        let mut imei = [0u8; 15];

        let mut sv: Option<u8> = None;

        let mut i = 0;

        for _ in 0..8 {
            let first_digit = buffer[pos] & 0xF;
            let second_digit = (buffer[pos] >> 4) & 0xF;

            imei[i] = first_digit;
            i = i + 1;

            if i == 15 {
                // This is either the SV or a stop bit
                if second_digit != 0xF {
                    // It has an SV
                    sv = Some(second_digit);
                }
            }
            else {
                imei[i] = second_digit;
            }
            
            i = i + 1;

            pos = pos + 1;
        }

        Some(
            (
                InformationElement {
                    imei,
                    sv,
                    instance,
                },
                (length + 4) as usize
            )
        )
    }

    pub fn generate_tbcd_imei(&self) -> [u8; 8] {
        let mut tbcd_imei = [0xFF; 8];

        let mut tbcd_index = 0;
        let mut imei_index = 0;

        while imei_index < self.imei.len() {
            tbcd_imei[tbcd_index] = self.imei[imei_index] & 0xF;
            imei_index = imei_index + 1;

            if imei_index < self.imei.len() {
                tbcd_imei[tbcd_index] = tbcd_imei[tbcd_index] | ((self.imei[imei_index] & 0xF) << 4);
                imei_index = imei_index + 1;
            }
            else {
                // For an imei with no sv, digits the high nibble should be filled with 0xF
                if let Some(sv) = self.sv {
                    tbcd_imei[tbcd_index] = tbcd_imei[tbcd_index] | ((sv & 0xF) << 4);
                }
                else {
                    tbcd_imei[tbcd_index] = tbcd_imei[tbcd_index] | 0xF0;
                }   
            }

            tbcd_index = tbcd_index + 1;
        }

        tbcd_imei
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::MEI
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

        4+8
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

        let tbcd_imei = self.generate_tbcd_imei();

        for i in 0..tbcd_imei.len() {
            buffer[pos] = tbcd_imei[i];
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
    fn test_generate() {
        let mut buffer = [0; MTU];

        let imei: [u8; 15] = [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6];

        let ie = InformationElement::new(imei, None, 0).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::MEI as u8,
            0, 8, // Length
            0, // Spare
            0x21, 0x43, 0x65, 0x87, 0x19, 0x32, 0x54, 0xF6 // IMEI
        ]);

        let ie = InformationElement::new(imei, Some(1), 0).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::MEI as u8,
            0, 8, // Length
            0, // Spare
            0x21, 0x43, 0x65, 0x87, 0x19, 0x32, 0x54, 0x16 // IMEISV
        ]);
    }

    #[test]
    fn test_length() {
        let imei: [u8; 15] = [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6];

        let ie = InformationElement::new(imei, None, 0).unwrap();
        assert_eq!(ie.length(), 8+4);
    }

    #[test]
    fn test_message_type() {
        let imei: [u8; 15] = [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6];

        let ie = InformationElement::new(imei, None, 0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::MEI as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::MEI as u8,
            0, 8, // Length
            0, // Spare
            0x21, 0x43, 0x65, 0x87, 0x19, 0x32, 0x54, 0x16 // IMEISV
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.imei, [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6]);
            assert!(ie.sv.is_some());
            assert_eq!(ie.sv.unwrap(), 1);
        }
        else {
            assert!(false);
        }

        let ie_bytes = [InformationElementType::MEI as u8,
            0, 8, // Length
            0, // Spare
            0x21, 0x43, 0x65, 0x87, 0x19, 0x32, 0x54, 0xF6 // IMEI
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.imei, [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6]);
            assert!(ie.sv.is_none());
        }
        else {
            assert!(false);
        }
    }
}