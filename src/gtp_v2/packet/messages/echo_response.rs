use super::{MessageTraits, MessageType};

use super::information_elements::{InformationElement, InformationElementTraits, recovery};

pub struct Message {
    /*
        --------------------------------------------|---------------------------|-------------------------                             
        Information Element                         |   Presence requirement    |   Reference 
        --------------------------------------------|---------------------------|-------------------------
        Recovery                                    |    Mandatory              |   8.5
        Sending Node Features                       |    Conditional Optional   |   
        Private Extensions                          |    Optional               |   
        --------------------------------------------|---------------------------|-------------------------
    */

    recovery: recovery::InformationElement
}

impl Message {
    pub fn new(recovery: recovery::InformationElement) -> Message {
        Message {
            recovery
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        let mut recovery: Option<recovery::InformationElement> = None;
        
        while pos < buffer.len()
        {
            if let Some((ie, ie_pos)) = InformationElement::parse(&buffer[pos..]){
                match ie {
                    InformationElement::Recovery(ie) => recovery = Some(ie),
                    _ =>  { /* Its an IE that we didn't expect. Just ignore it */ }
                }
                pos = pos + ie_pos;
            }
            else {
                // IE parsing failed
                pos = pos + InformationElement::skip_parsing(&buffer[pos..]);
            }
        }

        if recovery.is_some() {
                Some((
                    Message {
                        recovery: recovery.unwrap()
                    }, 
                    pos
                ))
        }
        else { None }
    }
}

impl MessageTraits for Message {
    fn message_type(&self) -> MessageType {
        MessageType::EchoResponse
    }

    fn length(&self) -> u16 {
        let mut length = 0;

        length = length + self.recovery.length();

        length
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        self.recovery.generate(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    use crate::gtp_v2::packet::messages::MessageTraits;
    use crate::gtp_v2::packet::messages::MessageType; 
    use crate::gtp_v2::packet::messages::information_elements::{InformationElementType};

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let m = Message::new(
            recovery::InformationElement::new(0xFF, 0).unwrap()
        );

        let pos = m.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::Recovery as u8,
            0, 1, // Length
            0, // Spare
            0xFF // Restart Counter
        ]);
    }
    
    #[test]
    fn test_length() {
        let m = Message::new(
            recovery::InformationElement::new(0xAB, 0).unwrap()
        );
        assert_eq!(m.length(), 5)
    }

    #[test]
    fn test_message_type() {
        let m = Message::new(
            recovery::InformationElement::new(0xCD, 0).unwrap()
        );
        assert_eq!(m.message_type() as u8, MessageType::EchoResponse as u8)
    }

    #[test]
    fn test_message_parse() {
        let message_bytes = [InformationElementType::Recovery as u8,
            0, 1, // Length
            0, // Spare
            0xAB, // Restart Counter
        ];

        if let Some((m, _pos)) = Message::parse(&message_bytes){
            assert_eq!(m.recovery.restart_counter, 0xAB);
        }

        let message_bytes = [InformationElementType::Recovery as u8,
            0, 1, // Length
            0, // Spare
            0xAB, // Restart Counter
            0xFF,
            0, 1, // Length
            0xF, 
            0xFF
        ];

        if let Some((m, _pos)) = Message::parse(&message_bytes){
            assert_eq!(m.recovery.restart_counter, 0xAB);
        }
    }
}
