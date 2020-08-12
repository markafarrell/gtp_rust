// use byteorder::{ByteOrder, NetworkEndian};

use super::{MessageTraits, MessageType};

use super::information_elements;

pub struct Message {
}

impl Message {
    pub fn new() -> Message {
        Message {}
    }
}

impl MessageTraits for Message {
    fn push_ie(&mut self, _ie: Box<dyn information_elements::InformationElementTraits>)
    {
        ()
    }

    fn pop_ie(&mut self) -> Option<Box<dyn information_elements::InformationElementTraits>>
    {
        None
    }

    fn attach_packet(&mut self, _packet: &[u8]) -> Result<usize,String>
    {
        Err("Packets cannot be attached to this message type".to_string())
    }

    fn message_type(&self) -> u8 {
        MessageType::EchoRequest as u8
    }
    fn length(&self) -> u16 {
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
        assert_eq!(m.message_type(), MessageType::EchoRequest as u8)
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}
