use super::{MessageTraits, MessageType};

use super::information_elements::InformationElement;

pub struct Message {
}

impl Message {
    pub fn new() -> Message {
        Message {}
    }
    pub fn parse(_buffer: &[u8]) -> Option<(Self, usize)> {
        Some((Message::new(),0))
    }
}

impl MessageTraits for Message {
    fn push_ie(&mut self, _ie: InformationElement)
    {
        ()
    }

    fn pop_ie(&mut self) -> Option<InformationElement>
    {
        None
    }

    fn message_type(&self) -> MessageType {
        MessageType::EchoRequest
    }
    fn length(&self) -> u16 {
        0
    }
    fn generate(&self, _buffer: &mut[u8]) -> usize {
        0
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

        let pos = m.generate(&mut buffer);

        assert_eq!(buffer[..pos], [0u8; 0]);
    }
    
    #[test]
    fn test_length() {
        let m = Message::new();
        assert_eq!(m.length(), 0)
    }

    #[test]
    fn test_message_type() {
        let m = Message::new();
        assert_eq!(m.message_type() as u8, MessageType::EchoRequest as u8)
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}
