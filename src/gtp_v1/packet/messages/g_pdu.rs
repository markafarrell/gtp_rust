// use byteorder::{ByteOrder, NetworkEndian};

use std::convert::TryInto;

use super::{MessageTraits, MessageType};

use super::information_elements;

pub struct Message {
    t_pdu: Vec<u8>
}

const MESSAGE_TYPE: u8 = MessageType::GPDU as u8;

impl Message {
    pub fn new() -> Self {
        Message {
            t_pdu: Vec::new()
        }
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

    fn attach_packet(&mut self, packet: &[u8]) -> Result<usize, String> {
        if packet.len() > 0xFFFF {
            return Err(format!("Packet to large. Length = {}", packet.len()));
        }

        self.t_pdu = Vec::new();
        
        for i in 0..packet.len()
        {
            self.t_pdu.push(packet[i]);
        }

        Ok(packet.len())
    }

    fn message_type(&self) -> u8 {
        MESSAGE_TYPE
    }
    fn length(&self) -> u16 {
        self.t_pdu.len().try_into().unwrap()
    }
    fn generate(&self, buffer: &mut[u8]) -> usize {
        for i in 0..self.t_pdu.len()
        {
            buffer[i] = self.t_pdu[i];
        }
        self.t_pdu.len()
    }
    fn parse(&mut self, _buffer: &[u8]) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    use crate::gtp_v1::packet::messages::MessageType; 

    #[test]
    fn test_attach_packet() {
        let icmpv4 = [
            0x45, 0x00, 0x00, 0x54, 0xaf, 0x2a, 0x40, 0x00,
            0x3f, 0x01, 0xba, 0xcc, 0xc0, 0xa8, 0x00, 0xfa,
            0x08, 0x08, 0x08, 0x08, 
            0x08, 0x00, 0xa9, 0xfe, 0x03, 0xe9, 0x00, 0x01,
            0x5a, 0x5f, 0x33, 0x5f, 0x00, 0x00, 0x00, 0x00,
            0xfd, 0x85, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37
        ];

        let mut m = Message::new();
        
        if let Ok(_) = m.attach_packet(&icmpv4) {
            assert!(true)
        }
        else {
            //Could not create message
            assert!(false)
        }

        let too_big = [ 0; 0x1FFFF ];

        let mut m = Message::new();
        
        if let Ok(_) = m.attach_packet(&too_big) {
            assert!(false)
        }
        else {
            // We have too big a packet. As a result attach_packet should fail
            assert!(true)
        }
    }

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let icmpv4 = [
            0x45, 0x00, 0x00, 0x54, 0xaf, 0x2a, 0x40, 0x00,
            0x3f, 0x01, 0xba, 0xcc, 0xc0, 0xa8, 0x00, 0xfa,
            0x08, 0x08, 0x08, 0x08, 
            0x08, 0x00, 0xa9, 0xfe, 0x03, 0xe9, 0x00, 0x01,
            0x5a, 0x5f, 0x33, 0x5f, 0x00, 0x00, 0x00, 0x00,
            0xfd, 0x85, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37
        ];

        let mut m = Message::new();
        
        if let Ok(_) = m.attach_packet(&icmpv4) {
            let end = m.generate(&mut buffer);

            for i in 0..end {
                if buffer[i] != icmpv4[i] {
                    println!("{} (actual) != {} (expected) at byte {}", buffer[i], icmpv4[i], i);
                    assert!(false);
                } 
            }
        }
        else {
            //Could not create message
            assert!(false)
        }
    }
    
    #[test]
    fn test_length() {
        let icmpv4 = [
            0x45, 0x00, 0x00, 0x54, 0xaf, 0x2a, 0x40, 0x00,
            0x3f, 0x01, 0xba, 0xcc, 0xc0, 0xa8, 0x00, 0xfa,
            0x08, 0x08, 0x08, 0x08, 
            0x08, 0x00, 0xa9, 0xfe, 0x03, 0xe9, 0x00, 0x01,
            0x5a, 0x5f, 0x33, 0x5f, 0x00, 0x00, 0x00, 0x00,
            0xfd, 0x85, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
            0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
            0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37
        ];

        let mut m = Message::new();
        
        if let Ok(_) = m.attach_packet(&icmpv4) {
            assert_eq!(m.length(), 84)
        }
        else {
            //Could not create message
            assert!(false)
        }
    }

    #[test]
    fn test_message_type() {
        let m = Message::new();

        assert_eq!(m.message_type(), MessageType::GPDU as u8)    
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}
