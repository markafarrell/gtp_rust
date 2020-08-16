use byteorder::{ByteOrder, NetworkEndian};

use super::{ExtensionHeaderTraits, ExtensionHeaderType};

pub struct ExtensionHeader {
    /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | Extension Header Length (1)                                   |
        2       | 0xFF                                                          |
        3       | 0xFF                                                          |
        4       | Next Extension Header Type                                    |
                |---------------------------------------------------------------|

    */
    next_extension_header_type: ExtensionHeaderType,
}

impl ExtensionHeader {
    pub fn new() -> ExtensionHeader {
        ExtensionHeader {
            next_extension_header_type: ExtensionHeaderType::NoMore
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;
        // Parse the length
        let length = buffer[0];
        pos = pos + 1;

        // Padding
        pos = pos + 2;

        // Read the next extension header type in last octet
        let next_extension_header_type: ExtensionHeaderType = buffer[pos].into();
        pos = pos + 1;

        Some(
            (
                ExtensionHeader {
                    next_extension_header_type: next_extension_header_type
                },
                pos
            )
        )
    }
}

impl ExtensionHeaderTraits for ExtensionHeader {
    fn extension_header_type(&self) -> ExtensionHeaderType {
        ExtensionHeaderType::MsInfoChange
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

        NetworkEndian::write_u16(&mut buffer[1..3], 0xFFFF);

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

        assert_eq!(buffer[..pos], [0x01, 0xFF, 0xFF, ExtensionHeaderType::NoMore as u8]);
    }
    
    #[test]
    fn test_set_next_extension_header_type() {
        let mut buffer = [0; MTU];

        let mut eh = ExtensionHeader::new();

        assert_eq!(eh.next_extension_header_type() as u8, ExtensionHeaderType::NoMore as u8);

        eh.set_next_extension_header_type(ExtensionHeaderType::SuspendReq);

        assert_eq!(eh.next_extension_header_type() as u8, ExtensionHeaderType::SuspendReq as u8);

        let pos = eh.generate(&mut buffer);

        assert_eq!(buffer[..pos], [0x01, 0xFF, 0xFF, ExtensionHeaderType::SuspendReq as u8]);
    }
    
    #[test]
    fn test_length() {
        let eh = ExtensionHeader::new();
        assert_eq!(eh.length(), 4)
    }

    #[test]
    fn test_message_type() {
        let eh = ExtensionHeader::new();
        assert_eq!(eh.extension_header_type() as u8, ExtensionHeaderType::MsInfoChange as u8)
    }

    #[test]
    fn test_message_parse() {
        let eh_bytes = [0x01, 0xFF, 0xFF, ExtensionHeaderType::SuspendReq as u8];

        let eh = ExtensionHeader::parse(&eh_bytes);

        if let Some((eh, pos)) = eh {
            assert_eq!(eh.next_extension_header_type() as u8, ExtensionHeaderType::SuspendReq as u8);
            assert_eq!(pos, 4)
        }
    }
}
