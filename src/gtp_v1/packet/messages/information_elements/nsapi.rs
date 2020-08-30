use super::{InformationElementTraits, InformationElementType};

pub struct InformationElement {
    /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (20)                                                  |
        2       |   X   |   X   |   X   |   X   | NSAPI                         |
                |---------------------------------------------------------------|
    */
    nsapi: u8
}

impl InformationElement {
    pub fn new(nsapi: u8) -> Result<Self, String> {
        // NSAPI can only be 4 bit i.e. max of 0xF(15)

        if nsapi > 0xF {
            return Err(format!("NSAPI is out of range. NSAPI = {}", nsapi));
        }

        Ok(InformationElement {
            nsapi: nsapi
        })
    }

    pub fn parse(_buffer: &[u8]) -> Option<(Self, usize)> {
        None
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::Nsapi
    }

    fn length(&self) -> u16 {
        2
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = 0;
        
        // Write the type
        buffer[pos] = self.information_element_type() as u8;

        pos = pos + 1;

        buffer[pos] = self.nsapi;

        pos = pos + 1;

        pos
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
        let invalid_nsapi: u8 = 0xFF;
        let ie = InformationElement::new(invalid_nsapi);

        match ie {
            Ok(_) => {
                assert!(false);
            }
            Err(e) => {
                println!("{}", e);
                assert!(true);
            }
        }

        let valid_nsapi: u8 = 0xF;
        let ie = InformationElement::new(valid_nsapi);

        match ie {
            Ok(_) => {
                assert!(true);
            }
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let nsapi: u8 = 0xF;
        let ie = InformationElement::new(nsapi);

        match ie {
            Ok(ie) => {
                let pos = ie.generate(&mut buffer);
                assert_eq!(buffer[..pos], [InformationElementType::Nsapi as u8, 0xF]);
            }
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
        
    }
    
    #[test]
    fn test_length() {
        let nsapi: u8 = 0xF;
        let ie = InformationElement::new(nsapi);

        match ie {
            Ok(ie) => {
                assert_eq!(ie.length(), 2)
            }
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_message_type() {
        let nsapi: u8 = 0xF;
        let ie = InformationElement::new(nsapi);

        match ie {
            Ok(ie) => {
                assert_eq!(ie.information_element_type() as u8, InformationElementType::Nsapi as u8)
            }
            Err(e) => {
                println!("{}", e);
                assert!(false);
            }
        }       
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}