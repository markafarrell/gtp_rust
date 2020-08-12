use byteorder::{ByteOrder, NetworkEndian};

use super::{ExtensionHeaderTraits, ExtensionHeaderType};

use std::convert::TryInto;

pub struct ExtensionHeader {
    /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | Extension Header Length (1)                                   |
        2       | PDCP PDU number                                               |
        3       | PDCP PDU number                                               |
        4       | Next Extension Header Type                                    |
                |---------------------------------------------------------------|

    */
    pdcp_pdu_number: u32, // Max size is 18 bits
    next_extension_header_type: ExtensionHeaderType,
}

impl ExtensionHeader {
    pub fn new() -> ExtensionHeader {
        ExtensionHeader {
            pdcp_pdu_number: 0x0000,
            next_extension_header_type: ExtensionHeaderType::NoMore
        }
    }

    pub fn set_pdcp_pdu_number(&mut self, value: u32) -> Result<u32,String> {
        if value > 0x3FFFF {
            return Err(format!("PDCP Number ({}) can not be > 0x3FFF.", value));
        }
        else
        {
            self.pdcp_pdu_number = value;
        }
        Ok(self.pdcp_pdu_number)
    }

    pub fn pdcp_pdu_number(&self) -> u32 {
        self.pdcp_pdu_number
    }
}

impl ExtensionHeaderTraits for ExtensionHeader {
    fn extension_header_type(&self) -> ExtensionHeaderType {
        ExtensionHeaderType::LongPdcpPduNumber
    }

    fn set_next_extension_header_type(&mut self, next_extension_header_type: ExtensionHeaderType) {
        self.next_extension_header_type = next_extension_header_type;
    }

    fn next_extension_header_type(&self) -> ExtensionHeaderType {
        self.next_extension_header_type
    }

    fn length(&self) -> u8 {
        2
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        // Write the length
        buffer[0] = self.length();

        // Write high 2 bits of pdcp_pdu_number
        buffer[1] = ((self.pdcp_pdu_number >> 16 & 0xFF)).try_into().unwrap();
        
        // Write low 16 bits of pdcp_pdu_number
        NetworkEndian::write_u16(&mut buffer[2..4], (self.pdcp_pdu_number & 0xFFFF).try_into().unwrap());

        // Write next extension header type in last octet
        buffer[self.length() as usize * 4 - 1] = self.next_extension_header_type as u8;

        self.length() as usize * 4
    }
    
    fn parse(&mut self, _buffer: &[u8]) {
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    use crate::gtp_v1::packet::header::extension_headers::{ExtensionHeaderTraits, ExtensionHeaderType};

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let eh = ExtensionHeader::new();

        let end = eh.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            0x02, // Length
            0x00, 0x00, 0x00, // PDCP PDU Number
            0x00, 0x00, 0x00, // Spare
            ExtensionHeaderType::NoMore as u8
        ]);
    }
    
    #[test]
    fn test_set_next_extension_header_type() {
        let mut buffer = [0; MTU];

        let mut eh = ExtensionHeader::new();

        assert_eq!(eh.next_extension_header_type() as u8, ExtensionHeaderType::NoMore as u8);

        eh.set_next_extension_header_type(ExtensionHeaderType::MsInfoChange);

        assert_eq!(eh.next_extension_header_type() as u8, ExtensionHeaderType::MsInfoChange as u8);

        let end = eh.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            0x02, // Length
            0x00, 0x00, 0x00, // PDCP PDU Number
            0x00, 0x00, 0x00, // Spare
            ExtensionHeaderType::MsInfoChange as u8
        ]);
    }
    
    #[test]
    fn test_set_pdcp_pdu_number() {
        let mut buffer = [0; MTU];

        let mut eh = ExtensionHeader::new();

        if let Ok(_) = eh.set_pdcp_pdu_number(0x3FFFF)
        {
            assert_eq!(eh.pdcp_pdu_number(), 0x3FFFF);
            
            let end = eh.generate(&mut buffer);

            assert_eq!(buffer[..end], [
                0x02, // Length
                0x03, 0xFF, 0xFF, // PDCP PDU Number
                0x00, 0x00, 0x00, // Spare
                ExtensionHeaderType::NoMore as u8
            ]);
        }
        else
        {
            // We can set PDCP PDU Numbers between 0 and 0x3FFFF
            assert!(false);
        }

        if let Ok(_) = eh.set_pdcp_pdu_number(0xFFFFF)
        {
            // We shouldn't be allowed to set this PDCP PDU Number
            assert!(false);
        }
        else
        {
            // We Should get an error
            assert!(true)
        }

        if let Ok(_) = eh.set_pdcp_pdu_number(0x1234)
        {
            assert_eq!(eh.pdcp_pdu_number(), 0x1234);
            
            let end = eh.generate(&mut buffer);
            
            assert_eq!(buffer[..end], [
                0x02, // Length
                0x00, 0x12, 0x34, // PDCP PDU Number
                0x00, 0x00, 0x00, // Spare
                ExtensionHeaderType::NoMore as u8
            ]);
        }
        else
        {
            // We can set PDCP PDU Numbers between 0 and 0x3FFFF
            assert!(false);
        }
    }

    #[test]
    fn test_length() {
        let eh = ExtensionHeader::new();
        assert_eq!(eh.length(), 2)
    }

    #[test]
    fn test_message_type() {
        let eh = ExtensionHeader::new();
        assert_eq!(eh.extension_header_type() as u8, ExtensionHeaderType::LongPdcpPduNumber as u8)
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}
