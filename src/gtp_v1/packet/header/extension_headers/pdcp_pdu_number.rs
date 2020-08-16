use byteorder::{ByteOrder, NetworkEndian};

use super::{ExtensionHeaderTraits, ExtensionHeaderType};

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
    pdcp_pdu_number: u16,
    next_extension_header_type: ExtensionHeaderType,
}

impl ExtensionHeader {
    pub fn new() -> ExtensionHeader {
        ExtensionHeader {
            pdcp_pdu_number: 0x0000,
            next_extension_header_type: ExtensionHeaderType::NoMore
        }
    }

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;
        // Parse the length
        let length = buffer[0];
        pos = pos + 1;

        let pdcp_pdu_number = NetworkEndian::read_u16(&buffer[1..3]);
        pos = pos + 2;

        // Read the next extension header type in last octet
        let next_extension_header_type: ExtensionHeaderType = buffer[pos].into();
        pos = pos + 1;

        Some(
            (
                ExtensionHeader {
                    pdcp_pdu_number: pdcp_pdu_number,
                    next_extension_header_type: next_extension_header_type
                },
                pos
            )
        )
    }

    pub fn set_pdcp_pdu_number(&mut self, value: u16) {
        self.pdcp_pdu_number = value;
    }

    pub fn pdcp_pdu_number(&self) -> u16 {
        self.pdcp_pdu_number
    }
}

impl ExtensionHeaderTraits for ExtensionHeader {
    fn extension_header_type(&self) -> ExtensionHeaderType {
        ExtensionHeaderType::PdcpPduNum
    }

    fn set_next_extension_header_type(&mut self, next_extension_header_type: ExtensionHeaderType) {
        self.next_extension_header_type = next_extension_header_type;
    }

    fn next_extension_header_type(&self) -> ExtensionHeaderType {
        self.next_extension_header_type
    }

    fn length(&self) -> u8 {
        4
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        // Write the length
        buffer[0] = self.length()/4;

        NetworkEndian::write_u16(&mut buffer[1..3], self.pdcp_pdu_number);

        // Write next extension header type in last octet
        buffer[self.length() as usize - 1] = self.next_extension_header_type as u8;

        self.length() as usize
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

        let pos = eh.generate(&mut buffer);

        assert_eq!(buffer[..pos], [0x01, 0x00, 0x00, ExtensionHeaderType::NoMore as u8]);
    }
    
    #[test]
    fn test_set_next_extension_header_type() {
        let mut buffer = [0; MTU];

        let mut eh = ExtensionHeader::new();

        assert_eq!(eh.next_extension_header_type() as u8, ExtensionHeaderType::NoMore as u8);

        eh.set_next_extension_header_type(ExtensionHeaderType::MsInfoChange);

        assert_eq!(eh.next_extension_header_type() as u8, ExtensionHeaderType::MsInfoChange as u8);

        let pos = eh.generate(&mut buffer);

        assert_eq!(buffer[..pos], [0x01, 0x00, 0x00, ExtensionHeaderType::MsInfoChange as u8]);
    }
    
    #[test]
    fn test_set_pdcp_pdu_number() {
        let mut buffer = [0; MTU];

        let mut eh = ExtensionHeader::new();

        eh.set_pdcp_pdu_number(0x1234);

        assert_eq!(eh.pdcp_pdu_number(), 0x1234);

        let pos = eh.generate(&mut buffer);

        assert_eq!(buffer[..pos], [0x01, 0x12, 0x34, ExtensionHeaderType::NoMore as u8]);
    }

    #[test]
    fn test_length() {
        let eh = ExtensionHeader::new();
        assert_eq!(eh.length(), 4)
    }

    #[test]
    fn test_message_type() {
        let eh = ExtensionHeader::new();
        assert_eq!(eh.extension_header_type() as u8, ExtensionHeaderType::PdcpPduNum as u8)
    }

    #[test]
    fn test_message_parse() {
        let eh_bytes = [0x01, 0x12, 0x34, ExtensionHeaderType::NoMore as u8];

        let eh = ExtensionHeader::parse(&eh_bytes);

        if let Some((eh, pos)) = eh {
            assert_eq!(eh.next_extension_header_type() as u8, ExtensionHeaderType::NoMore as u8);
            assert_eq!(eh.pdcp_pdu_number(), 0x1234);
            assert_eq!(pos, 4)
        }
    }
}
