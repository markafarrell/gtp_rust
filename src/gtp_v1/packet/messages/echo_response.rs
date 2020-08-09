// use byteorder::{ByteOrder, NetworkEndian};

use super::{MessageTraits, MessageType};

pub struct Message {
    /*                                  
                                        Bits
            |---------------------------------------------------------------| 
    Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
    (index) |---------------------------------------------------------------|
    1 (0)   | Version               | PT    | (*)   | E     | S     | PN    | 
    2 (1)   | Message Type (2)                                              |
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
}

const MESSAGE_TYPE: u8 = MessageType::EchoResponse as u8;

impl Message {
    pub fn new() -> Message {
        Message {}
    }
}

impl MessageTraits for Message {
    fn message_type(&self) -> u8 {
        MESSAGE_TYPE
    }
    fn length(&self) -> u8 {
        0
    }
    fn generate(&self, _buffer: &mut[u8]) -> usize {
        0
    }
    fn parse(&mut self, _buffer: &[u8]) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    use crate::gtp_v1::packet::messages::MessageTraits;
    use crate::gtp_v1::packet::messages::MessageType; 

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let m = Message::new();

        let end = m.generate(&mut buffer);

        assert_eq!(buffer[..end], []);
    }
    
    #[test]
    fn test_length() {
        let m = Message::new();
        assert_eq!(m.length(), 0)
    }

    #[test]
    fn test_message_type() {
        let m = Message::new();
        assert_eq!(m.message_type(), MessageType::EchoResponse as u8)
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}
