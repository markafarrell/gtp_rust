use super::{InformationElementTraits, InformationElementType, LENGTH};

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
        2       | IMSI[1]                       | IMSI[0]                       |
        3       | IMSI[3]                       | IMSI[2]                       |
        4       | IMSI[5]                       | IMSI[4]                       |
        5       | IMSI[7]                       | IMSI[6]                       |
        6       | IMSI[9]                       | IMSI[8]                       |
        7       | IMSI[11]                      | IMSI[10]                      |
        8       | IMSI[13]                      | IMSI[12]                      |
        9       | 0xF                           | IMSI[14]                      |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub imsi: [u8; 15],
}

impl InformationElement {
    pub fn new(imsi: &str, instance: u8) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else if imsi.len() != 15 {
            Err("IMSI is not the correct number of digits".to_string())
        }
        else {
            let mut parsed_imsi = [0; 15];

            for (i, c) in imsi.chars().enumerate() {
                let digit = c.to_string().parse::<u8>();
    
                let digit = match digit {
                    Ok(n) => n,
                    Err(_e) => {
                        return Err("Could not parse IMSI".to_string())
                    }
                };
    
                parsed_imsi[i] = digit;
            }
    
            Ok(InformationElement {
                imsi: parsed_imsi,
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

        let mut i = 0;

        let mut imsi = [0u8; 15];

        while pos < (length+4) as usize {

            let first_digit = buffer[pos] & 0xF;
            let second_digit = (buffer[pos] >> 4) & 0xF;

            imsi[i] = first_digit;
            i = i + 1;

            if second_digit != 0xF {
                imsi[i] = second_digit;
            }
            i = i + 1;

            pos = pos + 1;
        }

        Some(
            (
                InformationElement {
                    imsi,
                    instance,
                },
                pos
            )
        )
    }

    pub fn generate_tbcd_imsi(&self) -> [u8; 8] {
        let mut tbcd_imsi = [0xFF; 8];

        let mut tbcd_index = 0;
        let mut imsi_index = 0;

        while imsi_index < self.imsi.len() {
            tbcd_imsi[tbcd_index] = self.imsi[imsi_index] & 0xF;
            imsi_index = imsi_index + 1;

            if imsi_index < self.imsi.len() {
                tbcd_imsi[tbcd_index] = tbcd_imsi[tbcd_index] | ((self.imsi[imsi_index] & 0xF) << 4);
                imsi_index = imsi_index + 1;
            }
            else {
                // For an odd number of imsi digits the high nibble should be filled with 0xF
                tbcd_imsi[tbcd_index] = tbcd_imsi[tbcd_index] | 0xF0;
            }

            tbcd_index = tbcd_index + 1;
        }

        tbcd_imsi
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::IMSI
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
        8+4
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

        let tbcd_imsi = self.generate_tbcd_imsi();

        for i in 0..tbcd_imsi.len() {
            buffer[pos] = tbcd_imsi[i];
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
        let imsi_valid = "505013485090404";
        let imsi_ie = InformationElement::new(imsi_valid, 0);

        match imsi_ie {
            Ok(_) => {
                assert_eq!(true, true);
            }
            Err(e) => {
                println!("{}", e);
                assert_eq!(false, true)
            }
        }

        let imsi_invalid = "5f5013485090404";
        let imsi_ie = InformationElement::new(imsi_invalid, 0);

        match imsi_ie {
            Ok(_) => {
                assert_eq!(false, true)
            }
            Err(e) => {
                assert_eq!(e, "Could not parse IMSI".to_string())
            }
        }

        let imsi_invalid = "50501348509040";
        let imsi_ie = InformationElement::new(imsi_invalid, 0);

        match imsi_ie {
            Ok(_) => {
                assert_eq!(false, true)
            }
            Err(e) => {
                assert_eq!(e, "IMSI is not the correct number of digits".to_string())
            }
        }
    }

    #[test]
    fn test_generate_tbcd_imsi()
    {
        let imsi = "505013485090404";
        let imsi_ie = InformationElement::new(imsi, 0);

        assert_eq!(imsi_ie.is_ok(), true);

        if let Ok(i) = imsi_ie {
            assert_eq!(i.generate_tbcd_imsi(), [0x05, 0x05, 0x31, 0x84, 0x05, 0x09, 0x04, 0xF4]);
        }
        else {
            assert_eq!(false, true)
        }      
    }

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let imsi = "505013485090404";
        let imsi_ie = InformationElement::new(imsi, 0);

        match imsi_ie {
            Ok(i) => {
                let pos = i.generate(&mut buffer);
                assert_eq!(buffer[..pos], [
                    InformationElementType::IMSI as u8,
                    0, 8, // Length
                    0, // Spare
                    0x05, 0x05, 0x31, 0x84, 0x05, 0x09, 0x04, 0xF4]);
            }
            Err(e) => {
                println!("{}", e);
                assert_eq!(false, true)
            }
        }
    }
    
    #[test]
    fn test_length() {
        let imsi = "505013485090404";
        let ie = InformationElement::new(imsi, 0);

        if let Ok(i) = ie {
            assert_eq!(i.length(), 8+4)
        }
        else {
            assert_eq!(false, true)
        }   
    }

    #[test]
    fn test_message_type() {
        let imsi = "505013485090404";
        let ie = InformationElement::new(imsi, 0);

        if let Ok(i) = ie {
            assert_eq!(i.information_element_type() as u8, InformationElementType::IMSI as u8)
        }
        else {
            assert_eq!(false, true);
        }
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [
            InformationElementType::IMSI as u8,
            0, 8, // Length
            0, // Spare
            0x05, 0x05, 0x31, 0x84, 0x05, 0x09, 0x04, 0xF4
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.imsi, [5, 0, 5, 0, 1, 3, 4, 8, 5, 0, 9, 0, 4, 0, 4]);
        }
        else {
            assert!(false);
        }
    }
}