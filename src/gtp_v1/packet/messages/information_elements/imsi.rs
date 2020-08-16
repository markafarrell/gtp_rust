use super::{InformationElementTraits, InformationElementType};

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (2)                                                   |
        2       | IMSI[0]                       | 0xF                           |
        3       | IMSI[2]                       | IMSI[1]                       |
        4       | IMSI[4]                       | IMSI[3]                       |
        5       | IMSI[6]                       | IMSI[5]                       |
        6       | IMSI[8]                       | IMSI[7]                       |
        7       | IMSI[10]                      | IMSI[9]                       |
        8       | IMSI[12]                      | IMSI[11]                      |
        9       | IMSI[14]                      | IMSI[13]                      |
                |---------------------------------------------------------------|
    */

    imsi: [u8; 15]
}

impl InformationElement {
    pub fn new(imsi: &str) -> Result<Self, String> {

        if imsi.len() != 15 {
            return Err("IMSI is not the correct number of digits".to_string());
        }

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
            imsi: parsed_imsi
        })
    }

    pub fn generate_tbcd_imsi(&self) -> [u8; 8] {
        let mut tbcd_imsi = [0xFF; 8];

        for n in 0..8 as u8 {
            let mut first_nibble = 0xF;
            let mut second_nibble = 0xF;

            let first_nibble_idx: usize = (n*2).into();
            let second_nibble_idx = first_nibble_idx.checked_sub(1);
           
            if let Some(i) = second_nibble_idx {
                // This will only happen if checked_sub worked i.e. second_nibble_idx >= 0
                if i <= 14 {
                    second_nibble = self.imsi[i];
                }
            }

            if first_nibble_idx <= 14 {
                first_nibble = self.imsi[first_nibble_idx];
            }

            tbcd_imsi[n as usize] = (first_nibble << 4) | second_nibble;
        }
        tbcd_imsi
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::Imsi
    }

    fn length(&self) -> u16 {
        9
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = 0;
        
        // Write the type
        buffer[pos] = self.information_element_type() as u8;

        pos = pos + 1;

        let tbcd_imsi = self.generate_tbcd_imsi();

        for i in 0..tbcd_imsi.len() {
            buffer[pos] = tbcd_imsi[i];
            pos = pos + 1;
        }

        pos
    }
    
    fn parse(&mut self, _buffer: &[u8]) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    use crate::gtp_v1::packet::messages::information_elements::InformationElementType;

    #[test]
    fn test_new()
    {
        let imsi_valid = "505013485090404";
        let imsi_ie = InformationElement::new(imsi_valid);

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
        let imsi_ie = InformationElement::new(imsi_invalid);

        match imsi_ie {
            Ok(_) => {
                assert_eq!(false, true)
            }
            Err(e) => {
                assert_eq!(e, "Could not parse IMSI".to_string())
            }
        }

        let imsi_invalid = "50501348509040";
        let imsi_ie = InformationElement::new(imsi_invalid);

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
        let imsi_ie = InformationElement::new(imsi);

        assert_eq!(imsi_ie.is_ok(), true);

        if let Ok(i) = imsi_ie {
            assert_eq!(i.generate_tbcd_imsi(), [0x5F, 0x50, 0x10, 0x43, 0x58, 0x90, 0x40, 0x40]);
        }
        else {
            assert_eq!(false, true)
        }      
    }

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let imsi = "505013485090404";
        let imsi_ie = InformationElement::new(imsi);

        match imsi_ie {
            Ok(i) => {
                let pos = i.generate(&mut buffer);
                assert_eq!(buffer[..pos], [InformationElementType::Imsi as u8, 0x5F, 0x50, 0x10, 0x43, 0x58, 0x90, 0x40, 0x40]);
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
        let ie = InformationElement::new(imsi);

        if let Ok(i) = ie {
            assert_eq!(i.length(), 9)
        }
        else {
            assert_eq!(false, true)
        }   
    }

    #[test]
    fn test_message_type() {
        let imsi = "505013485090404";
        let ie = InformationElement::new(imsi);

        if let Ok(i) = ie {
            assert_eq!(i.information_element_type() as u8, InformationElementType::Imsi as u8)
        }
        else {
            assert_eq!(false, true);
        }
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}