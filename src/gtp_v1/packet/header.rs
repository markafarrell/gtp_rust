use crate::field::*;

use byteorder::{ByteOrder, NetworkEndian};

use super::messages::MessageType;

pub mod extension_headers;

/*                                  
                                        Bits
            |---------------------------------------------------------------| 
    Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
    (index) |---------------------------------------------------------------|
    1 (0)   | Version               | PT    | (*)   | E     | S     | PN    | 
    2 (1)   | Message Type                                                  |
    3 (2)   | Length (1st Octet)                                            |
    4 (3)   | Length (2nd Octet)                                            |
    5 (4)   | Tunnel Endpoint Identifier (1st Octet)                        |
    6 (5)   | Tunnel Endpoint Identifier (2nd Octet)                        |
    7 (6)   | Tunnel Endpoint Identifier (3rd Octet)                        |
    8 (7)   | Tunnel Endpoint Identifier (4th Octet)                        |
    9 (8)   | Sequence Number (1st Octet)                                   |
    10 (9)  | Sequence Number (2nd Octet)                                   |
    11 (10) | N-PDU Number                                                  |
    12 (11) | Next Extension Header Type                                    |
            |---------------------------------------------------------------|
*/

pub const LENGTH: Field = 2..4;
pub const TEID: Field = 4..8;

pub struct Header {
    // MANDATORY FIELDS
    version: u8, // Always present, always set to 1
    message_type: MessageType, // Always present
    pt: u8, // Protocol Type: 0 = GTP', 1 = GTP
    e: u8, // Extenstion Header Flag: 0 = Not present, 1 = Present
    s: u8, // Sequence Number Flag: 0 = Not present, 1 = Present
    pn: u8, // N-PDU Number Flag: 0 = Not present, 1 = Present
    teid: u32, /* Tunnel Endpoint Identifier (TEID) */
    payload_length: u16, /* This is the length of any payload associated with the packet that
        this header is attached to */

    // OPTIONAL FIELDS
    sequence_number: u16, /* It is used as a transaction identity for signalling messages
        having a response message defined for a request message, that is the Sequence Number
        value is copied from the request to the response message header. In the user plane,
        an increasing sequence number for TPDUs is transmitted via GTP-U tunnels, when
        transmission order must be preserved. */
    n_pdu_number: u8, /* This field is used at the Inter SGSN Routeing Area Update procedure
        and some inter-system handover procedures (e.g. between 2G and 3G radio access
        networks). This field is used to co-ordinate the data transmission for acknowledged
        mode of communication between the MS and the SGSN. The exact meaning ofthis field
        depends upon the scenario. (For example, for GSM/GPRS to GSM/GPRS, the SNDCP N-PDU
        number is present in this field). */

    pub extension_headers: Vec<Box<dyn extension_headers::ExtensionHeaderTraits>>
}

impl Header {
    pub fn new(message_type: MessageType) -> Self {
        Header {
            version: 1,
            message_type: message_type,
            payload_length: 0,
            pt: 1,
            e: 0,
            s: 0,
            pn: 0,
            teid: 0x00000000,
            sequence_number: 0,
            n_pdu_number: 0,
            extension_headers: Vec::new()
        }
    }

    pub fn message_type(&self) -> MessageType {
        self.message_type
    }

    fn length(&self) -> u16 {
        /* Length of Payload in octets. i.e. the rest of the packet following the 
        mandatory part of the GTP header (that is the first 8 octets). The Sequence Number, 
        the N-PDU Number or any Extension headers shall be considered to be part of the 
        payload */

        // This returns the length of the optional parts of the header
        let mut length = self.payload_length;
        
        if self.s == 1 {
            length = length + 2; // 2 octets for sequence number
        }

        if self.pn == 1 {
            length = length + 1; // 1 octet for N_PDU number
        }

        if self.e == 1 {
            length = length + 1; // 1 octet for next extension header
        }

        for e in self.extension_headers.iter() {
            // length of extension headers is in multiples of 4 octets therfore we need to multiply by 4 here
            length = length + (e.length() * 4) as u16;
        }

        length
    }

    pub fn set_teid(&mut self, teid: u32) {
        self.teid = teid;
    }

    pub fn teid(&self) -> u32 {
        self.teid
    }

    pub fn enable_sequence_number(&mut self) {
        self.s = 1;
    }

    pub fn disable_sequence_number(&mut self) {
        self.s = 0;
    }

    pub fn set_sequence_number(&mut self, sequence_number: u16) {
        self.sequence_number = sequence_number;
    }

    pub fn sequence_number(&mut self) -> u16 {
        self.sequence_number
    }

    pub fn enable_n_pdu_number(&mut self) {
        self.pn = 1;
    }

    pub fn disable_n_pdu_number(&mut self) {
        self.pn = 0;
    }

    pub fn set_n_pdu_number(&mut self, n_pdu_number: u8) {
        self.n_pdu_number = n_pdu_number;
    }

    pub fn n_pdu_number(&mut self) -> u8 {
        self.n_pdu_number
    }

    pub fn push_extension_header(&mut self, extension_header: Box<dyn extension_headers::ExtensionHeaderTraits>) {
        let len = self.extension_headers.len();

        if len > 0 {
            // Set next_extension header field 
            self.extension_headers[len-1].set_next_extension_header_type(extension_header.extension_header_type());
        }

        // We now have extension headers. Set E flag to 1
        self.e = 1;

        self.extension_headers.push(extension_header);
    }

    pub fn pop_extension_header(&mut self) -> Option<Box<dyn extension_headers::ExtensionHeaderTraits>> {
        let extension_header = self.extension_headers.pop();

        if extension_header.is_some() {
            let len = self.extension_headers.len();

            if len > 0 {
                //Set new end of list to ExtensionHeaderType::NoMore as u8 for next_extension_header_type
                self.extension_headers[len-1].set_next_extension_header_type(extension_headers::ExtensionHeaderType::NoMore);
            }
            else {
                // No extension headers left. Set the E flag to 0
                self.e = 0
            }
        }
        
        extension_header
    }

    pub fn set_payload_length(&mut self, payload_length: u16) {
        self.payload_length = payload_length;
    }

    fn generate_flags(&self) -> u8 {
        (self.version << 5) | (self.pt << 4) | (0 << 3) | (self.e << 2) | (self.s << 1) | self.pn
    }

    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        buffer[0] = self.generate_flags();

        buffer[1] = self.message_type as u8;

        NetworkEndian::write_u16(&mut buffer[LENGTH],self.length());

        NetworkEndian::write_u32(&mut buffer[TEID],self.teid);

        // Optional fields start at index 8
        let mut end: usize = 8;

        if self.s == 1 {
            NetworkEndian::write_u16(&mut buffer[end..],self.sequence_number);
            end = end + 2;
        }

        if self.pn == 1 {
            buffer[end] = self.n_pdu_number;
            end = end + 1;
        }

        if self.e == 1 {
            if self.extension_headers.len() > 0 {
                // We write the type of the first extension header in the next_extension_header_type field here
                buffer[end] = self.extension_headers[0].extension_header_type() as u8;
                end = end + 1;
            }
            else {
                // This should never happen as we shouldnt have e set without anything in the extenstion headers vector
                // however, when/if this happens we should write ExtensionHeaderType::NoMore as u8 in the next_extension_header_type field
                buffer[end] = extension_headers::ExtensionHeaderType::NoMore as u8;
                end = end + 1;
            }
            for e in self.extension_headers.iter() {
                let extenstion_header_size = e.generate(&mut buffer[end..]);
                end = end + extenstion_header_size;
            }
        }

        end
    }
    
    pub fn parse(&mut self, _buffer: &[u8]) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::MTU;

    use crate::gtp_v1::packet::messages::MessageType; 
    use crate::gtp_v1::packet::header::extension_headers::{
        mbms_support_indication,
        suspend_request,
        pdcp_pdu_number,
        ExtensionHeaderType
    };

    #[test]
    fn test_message_type() {
        let h = Header::new(MessageType::EchoRequest);
        assert_eq!(h.message_type() as u8, MessageType::EchoRequest as u8);

        let h = Header::new(MessageType::EchoResponse);
        assert_eq!(h.message_type() as u8, MessageType::EchoResponse as u8);
    }

    #[test]
    fn test_generate_simple() {
        let mut buffer = [0; MTU];

        let h = Header::new(MessageType::EchoRequest);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);
    }
    
    #[test]
    fn test_teid() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        h.set_teid(0x12345678);

        assert_eq!(h.teid(), 0x12345678);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x12, 0x34, 0x56, 0x78
            ]);
    }

    #[test]
    fn test_sequence_number() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        h.set_sequence_number(0x1234);

        assert_eq!(h.sequence_number(), 0x1234);

        let end = h.generate(&mut buffer);

        // We haven't enabled SN so it shouldn't be output
        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);

        h.enable_sequence_number();

        assert_eq!(h.length(), 2);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0010, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x02,
            /* TEID */0x00, 0x00, 0x00, 0x00,
            /* Sequence Number */ 0x12, 0x34
            ]);

        h.disable_sequence_number();

        let end = h.generate(&mut buffer);

        // We haven't enabled SN so it shouldn't be output
        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);
    }

    #[test]
    fn test_n_pdu_number() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        h.set_n_pdu_number(0x12);

        assert_eq!(h.n_pdu_number(), 0x12);

        let end = h.generate(&mut buffer);

        // We haven't enabled N_PDU so it shouldn't be output
        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);

        h.enable_n_pdu_number();

        assert_eq!(h.length(), 1);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0001, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x01,
            /* TEID */0x00, 0x00, 0x00, 0x00,
            /* N_PDU Number */ 0x12
            ]);

        h.disable_n_pdu_number();

        let end = h.generate(&mut buffer);

        // We haven't enabled SN so it shouldn't be output
        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);
    }

    #[test]
    fn test_extension_headers() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        let mbms_si = Box::new(mbms_support_indication::ExtensionHeader::new());

        h.push_extension_header(mbms_si);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0100, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x05,
            /* TEID */ 0x00, 0x00, 0x00, 0x00,
            /* Next Extension Header Type */ ExtensionHeaderType::MbmsSi as u8,
            /* MBMS SI Ext Header */ 0x01, 0xFF, 0xFF, ExtensionHeaderType::NoMore as u8
            ]);

        let s_req = Box::new(suspend_request::ExtensionHeader::new());

        h.push_extension_header(s_req);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0100, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x09,
            /* TEID */ 0x00, 0x00, 0x00, 0x00,
            /* Next Extension Header Type */ ExtensionHeaderType::MbmsSi as u8,
            /* MBMS SI Ext Header */ 0x01, 0xFF, 0xFF, ExtensionHeaderType::SuspendReq as u8,
            /* Suspend Request Ext Header */ 0x01, 0xFF, 0xFF, ExtensionHeaderType::NoMore as u8
            ]);

        let mut pdcp_pdu_number = Box::new(pdcp_pdu_number::ExtensionHeader::new());

        pdcp_pdu_number.set_pdcp_pdu_number(0x1234);

        h.push_extension_header(pdcp_pdu_number);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0100, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x0d,
            /* TEID */ 0x00, 0x00, 0x00, 0x00,
            /* Next Extension Header Type */ ExtensionHeaderType::MbmsSi as u8,
            /* MBMS SI Ext Header */ 0x01, 0xFF, 0xFF, ExtensionHeaderType::SuspendReq as u8,
            /* Suspend Request Ext Header */ 0x01, 0xFF, 0xFF, ExtensionHeaderType::PdcpPduNum as u8,
            /* PDCP PDU Number Ext Header */ 0x01, 0x12, 0x34, ExtensionHeaderType::NoMore as u8
            ]);

        h.pop_extension_header();

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0100, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x09,
            /* TEID */ 0x00, 0x00, 0x00, 0x00,
            /* Next Extension Header Type */ ExtensionHeaderType::MbmsSi as u8,
            /* MBMS SI Ext Header */ 0x01, 0xFF, 0xFF, ExtensionHeaderType::SuspendReq as u8,
            /* Suspend Request Ext Header */ 0x01, 0xFF, 0xFF, ExtensionHeaderType::NoMore as u8
            ]);
    }

    #[test]
    fn test_length() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        h.set_payload_length(0x1234);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x12, 0x34,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);

        h.set_sequence_number(0x4567);
        h.enable_sequence_number();

        assert_eq!(h.length(), 0x1234+2);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0010, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x12, 0x36,
            /* TEID */0x00, 0x00, 0x00, 0x00,
            /* Sequence Number */ 0x45, 0x67
            ]);

        h.set_n_pdu_number(0x12);
        h.enable_n_pdu_number();

        assert_eq!(h.length(), 0x1234+2+1);
        let end = h.generate(&mut buffer);

        // We haven't enabled SN so it shouldn't be output
        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0011, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x12, 0x37,
            /* TEID */ 0x00, 0x00, 0x00, 0x00,
            /* Sequence Number */ 0x45, 0x67,
            /* N_PDU Number */ 0x12
            ]);
    }

    #[test]
    fn test_generate_everything() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        h.set_sequence_number(0x1234);
        h.set_n_pdu_number(0x12);

        h.enable_sequence_number();
        h.enable_n_pdu_number();

        assert_eq!(h.length(), 3);

        let end = h.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0011, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x03,
            /* TEID */0x00, 0x00, 0x00, 0x00,
            /* Sequence Number */ 0x12, 0x34,
            /* N_PDU Number */ 0x12
            ]);
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}
