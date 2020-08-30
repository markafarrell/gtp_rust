use std::convert::TryFrom;

use crate::field::*;
use byteorder::{ByteOrder, NetworkEndian};

use super::messages::MessageType;

/*                                  
                                        Bits
            |---------------------------------------------------------------| 
    Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
    (index) |---------------------------------------------------------------|
    1 (0)   | Version (0x2)         | P     | T     | MP    | Spare | Spare | 
    2 (1)   | Message Type                                                  |
    3 (2)   | Message Length (1st Octet)                                    |
    4 (3)   | Message Length (2nd Octet)                                    |
    5 (4)   | Tunnel Endpoint Identifier (1st Octet) * If T Flag is set     |
    6 (5)   | Tunnel Endpoint Identifier (2nd Octet) * If T Flag is set     |
    7 (6)   | Tunnel Endpoint Identifier (3rd Octet) * If T Flag is set     |
    8 (7)   | Tunnel Endpoint Identifier (4th Octet) * If T Flag is set     |
    9 (8)   | Sequence Number (1st Octet)                                   |
    10 (9)  | Sequence Number (2nd Octet)                                   |
    11 (10) | Sequence Number (3nd Octet)                                   |
    12 (11) | Message Priority              | Spare                         |
            |---------------------------------------------------------------|
*/

pub const LENGTH: Field = 2..4;
pub const TEID: Field = 4..8;

pub struct Header {
    // MANDATORY FIELDS
    version: u8, // Always present, always set to 1
    message_type: MessageType, // Always present
    p: u8, /* Piggyback: If set a piggybacked initial message is carried as a concatenation
        after a triggered response message and they share a common IP header. */
    t: u8, // If set to 1 then the TEID field is enabled
    mp: u8, // Message Priority flag
    payload_length: u16, /* This is the length of any payload associated with the packet that
        this header is attached to */

    sequence_number: u32, /* It is used as a transaction identity for signalling messages
        having a response message defined for a request message, that is the Sequence Number
        value is copied from the request to the response message header. In the user plane,
        an increasing sequence number for TPDUs is transmitted via GTP-U tunnels, when
        transmission order must be preserved. */

    // OPTIONAL FIELDS
    teid: u32, /* Tunnel Endpoint Identifier (TEID) */
    message_priority: u8
}

impl Header {
    pub fn new(message_type: MessageType) -> Self {
        Header {
            version: 2,
            message_type: message_type,
            payload_length: 0,
            p: 0,
            t: 0,
            mp: 0,
            teid: 0x00000000,
            sequence_number: 0,
            message_priority: 0,
        }
    }

    pub fn message_type(&self) -> MessageType {
        self.message_type
    }

    fn length(&self) -> u16 {
        /* Length of Payload in octets. i.e. the rest of the packet following the 
        mandatory part of the GTP header (that is the first 4 octets). The Sequence Number 
        and the TEID shall be considered to be part of the payload */

        // This returns the length of the attached message
        let mut length = self.payload_length;
        
        length = length + 3; // 3 octets for sequence number

        if self.t == 1 {
            length = length + 4; // 4 octets for TEID
        }

        length = length + 1; // This is for the spare octet at the pos of the header
        
        length
    }

    pub fn enable_teid(&mut self) {
        self.t = 1;
    }

    pub fn set_teid(&mut self, teid: u32) {
        self.teid = teid;
    }

    pub fn teid(&self) -> u32 {
        self.teid
    }

    pub fn set_sequence_number(&mut self, sequence_number: u32) -> Result<u32,String> {
        if sequence_number > 0xFFFFFF {
            // Sequence number can only be 3 octets
            return Err(format!("Sequence number ({}) too large.", sequence_number));
        }
        self.sequence_number = sequence_number;
        Ok(sequence_number)
    }

    pub fn sequence_number(&self) -> u32 {
        self.sequence_number
    }

    pub fn enable_message_priority(&mut self) {
        self.mp = 1;
    }

    pub fn disable_message_priority(&mut self) {
        self.mp = 0;
    }

    pub fn set_message_priority(&mut self, message_priority: u8) -> Result<u8,String> {
        if message_priority > 0xF {
            // Message Priority can only be 4 bits
            return Err(format!("Message Priority ({}) too large.", message_priority));
        }
        self.message_priority = message_priority;
        Ok(message_priority)
    }

    pub fn message_priority(&self) -> u8 {
        self.message_priority
    }

    pub fn set_payload_length(&mut self, payload_length: u16) {
        self.payload_length = payload_length;
    }

    fn generate_flags(&self) -> u8 {
        ((self.version & 0b111) << 5) | ((self.p & 0x1) << 4) | ((self.t & 0x1) << 3) | ((self.mp & 0x1) << 2)
    }

    fn parse_flags(flags: u8) -> (u8, u8, u8, u8) {
        let version = (flags >> 5) & 0b111;
        let p = (flags >> 4) & 0b1;
        let t = (flags >> 3) & 0b1;
        let mp = (flags >> 2) & 0b1;

        (version, p, t, mp)
    }

    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = 0;

        buffer[0] = self.generate_flags();
        pos = pos + 1;

        buffer[1] = self.message_type as u8;
        pos = pos + 1;

        NetworkEndian::write_u16(&mut buffer[LENGTH],self.length());
        pos = pos + 2;

        if self.t == 1 {
            NetworkEndian::write_u32(&mut buffer[pos..],self.teid);
            pos = pos + 4;
        }

        NetworkEndian::write_uint(&mut buffer[pos..], self.sequence_number as u64, 3);
        pos = pos + 3;

        if self.mp == 1 {
            buffer[pos] = self.message_priority << 4;
        }

        pos = pos + 1; // This is for the MP/spare octet

        pos
    }
    
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        let (version, p, t, mp) = Self::parse_flags(buffer[0]);
        pos = pos + 1;

        if version != 2 {
            // The packet isn't a GTPv2 packet
            return None;
        }

        if let Ok(message_type) = MessageType::try_from(buffer[1]) {
            pos = pos + 1;

            let mut h = Self::new(message_type);

            let _length = NetworkEndian::read_u16(&buffer[LENGTH]);
            pos = pos + 2;

            if t == 1 {
                h.set_teid(NetworkEndian::read_u32(&buffer[pos..]));
                h.enable_teid();
                pos = pos + 4;
            }

            if p == 1 {
                // We don't support piggyback
                return None;
            }

            h.set_sequence_number(NetworkEndian::read_uint(&buffer[pos..], 3) as u32).unwrap();
            pos = pos + 3;

            if mp == 1 {
                h.set_message_priority((buffer[pos] >> 4) & 0xF).unwrap();
                h.enable_message_priority();
            }

            pos = pos + 1; // This is for the MP/spare octet
    
            Some((h, pos))
        }
        else {
            None
        }

        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::MTU;

    use crate::gtp_v2::packet::messages::MessageType; 

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

        let pos = h.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0100_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x04,
            /* Sequence Number */ 0x00, 0x00, 0x00, 
            /* Spare */ 0x00
            ]
        );

        let mut buffer = [0; MTU];

        let h = Header::new(MessageType::EchoResponse);

        let pos = h.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0100_0000, 
            /* Message Type */ MessageType::EchoResponse as u8,
            /* Length */ 0x00, 0x04,
            /* Sequence Number */ 0x00, 0x00, 0x00, 
            /* Spare */ 0x00
            ]
        );
    }
    
    #[test]
    fn test_teid() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        h.set_teid(0x12345678);

        assert_eq!(h.teid(), 0x12345678);

        let pos = h.generate(&mut buffer);

        // We haven't enable the T flag so it shouldnt be generated

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0100_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x04,
            /* Sequence Number */ 0x00, 0x00, 0x00, 
            /* Spare */ 0x00
            ]
        );

        h.enable_teid();

        let pos = h.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0100_1000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x08,
            /* TEID */ 0x12, 0x34, 0x56, 0x78,
            /* Sequence Number */ 0x00, 0x00, 0x00, 
            /* Spare */ 0x00
            ]
        );
    }

    #[test]
    fn test_sequence_number() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        if let Err(e) = h.set_sequence_number(0xFFFFFFFF)
        {
            // We set too big a sequence number this should cause an error
            println!("{}", e);
            assert!(true);
        }
        else {
            assert!(false);
        }

        if let Err(e) = h.set_sequence_number(0x123456)
        {
            println!("{}", e);
            assert!(false);
        }
        else {
            assert_eq!(h.sequence_number(), 0x123456);

            let pos = h.generate(&mut buffer);

            assert_eq!(buffer[..pos], [
                /* Flags */ 0b0100_0000, 
                /* Message Type */ MessageType::EchoRequest as u8,
                /* Length */ 0x00, 0x04,
                /* Sequence Number */ 0x12, 0x34, 0x56, 
                /* Spare */ 0x00
                ]
            );
        }
    }

    #[test]
    fn test_length() {
        let mut buffer = [0; MTU];

        let mut h = Header::new(MessageType::EchoRequest);

        h.set_payload_length(0x1230);

        assert_eq!(h.length(), 0x1230+4);

        let pos = h.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0100_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x12, 0x34,
            /* Sequence Number */ 0x00, 0x00, 0x00, 
            /* Spare */ 0x00
            ]);

        h.set_teid(0x12345678);
        h.enable_teid();

        assert_eq!(h.length(), 0x1230+4+4);

        let pos = h.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0100_1000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x12, 0x38,
            /* TEID */ 0x12, 0x34, 0x56, 0x78,
            /* Sequence Number */ 0x00, 0x00, 0x00, 
            /* Spare */ 0x00
            ]);
    }

    #[test]
    fn test_message_parse() {
        let header_bytes = [
            /* Flags */ 0b0100_1000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x08,
            /* TEID */ 0x12, 0x34, 0x56, 0x78,
            /* Sequence Number */ 0x87, 0x65, 0x43, 
            /* Spare */ 0x00
            ];

        if let Some((h, _pos)) = Header::parse(&header_bytes) {
            assert_eq!(h.teid(), 0x12345678);
            assert_eq!(h.sequence_number(), 0x876543);
            assert_eq!(h.message_type() as u8, MessageType::EchoRequest as u8);
        }
    }
}
