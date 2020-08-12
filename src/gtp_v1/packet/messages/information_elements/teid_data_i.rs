use byteorder::{ByteOrder, NetworkEndian};

use crate::field::*;

use super::{InformationElementTraits, InformationElementType};

pub struct InformationElement {
    /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (16)                                                  |
        2       | TEID Octet 1                                                  |
        3       | TEID Octet 2                                                  |
        4       | TEID Octet 3                                                  |
        5       | TEID Octet 4                                                  |
                |---------------------------------------------------------------|
    */
    teid: u32
}

pub const TEID: Field = 1..5;

impl InformationElement {
    pub fn new(teid: u32) -> Self {
        InformationElement {
            teid: teid
        }
    }
    pub fn teid(&self) -> u32 {
        self.teid
    }
    pub fn set_teid(&mut self, teid: u32) {
        self.teid = teid;
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::TeidDataI
    }

    fn length(&self) -> u16 {
        5
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut end = 0;
        
        // Write the type
        buffer[end] = self.information_element_type() as u8;

        end = end + 1;

        NetworkEndian::write_u32(&mut buffer[TEID],self.teid);

        end = end + 4;

        end
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
    fn test_set_get_teid() {
        let mut buffer = [0; MTU];

        let teid: u32 = 0x12345678;
        let mut ie = InformationElement::new(teid);

        assert_eq!(ie.teid(), 0x12345678);

        ie.set_teid(0x87654321);

        assert_eq!(ie.teid(), 0x87654321);

        let end = ie.generate(&mut buffer);

        assert_eq!(buffer[..end], [InformationElementType::TeidDataI as u8, 0x87, 0x65, 0x43, 0x21]);
    }

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let teid: u32 = 0x12345678;
        let ie = InformationElement::new(teid);
        let end = ie.generate(&mut buffer);

        assert_eq!(buffer[..end], [InformationElementType::TeidDataI as u8, 0x12, 0x34, 0x56, 0x78]);
    }
    
    #[test]
    fn test_length() {
        let teid: u32 = 0x12345678;
        let ie = InformationElement::new(teid);

        assert_eq!(ie.length(), 5)
    }

    #[test]
    fn test_message_type() {
        let teid: u32 = 0x12345678;
        let ie = InformationElement::new(teid);

        assert_eq!(ie.information_element_type() as u8, InformationElementType::TeidDataI as u8)
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}