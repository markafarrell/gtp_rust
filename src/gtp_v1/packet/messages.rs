pub mod echo_request;
pub mod echo_response;
pub mod create_pdp_context_request;
pub mod information_elements;
pub mod g_pdu;

use information_elements::InformationElement;

#[derive(Copy, Clone)]
pub enum MessageType
{
    EchoRequest = 1,
    EchoResponse = 2,
    CreatePDPContextRequest = 16,
    GPDU = 255,
}

impl From<u8> for MessageType {
    fn from(v: u8) -> Self {
        match v {
            1 => MessageType::EchoRequest,
            2 => MessageType::EchoResponse,
            16 => MessageType::CreatePDPContextRequest,
            255 => MessageType::GPDU,
            _ => panic!(format!("Unsupported Message Type ({})",v ))
        }
    }
}

pub trait MessageTraits {
    fn message_type(&self) -> u8;
    fn length(&self) -> u16;
    fn generate(&self, buffer: &mut[u8]) -> usize;
    fn push_ie(&mut self, ie: InformationElement);
    fn pop_ie(&mut self) -> Option<InformationElement>;
}

pub enum Message {
    EchoRequest(echo_request::Message),
    EchoResponse(echo_response::Message),
    CreatePDPContextRequest(create_pdp_context_request::Message),
    GPDU(g_pdu::Message)
}

impl Message {
    pub fn new(message_type: MessageType) -> Self {
        match message_type {
            MessageType::EchoRequest => 
                Message::EchoRequest(echo_request::Message::new()),
            MessageType::EchoResponse => 
                Message::EchoResponse(echo_response::Message::new()),
            MessageType::CreatePDPContextRequest => 
                Message::CreatePDPContextRequest(create_pdp_context_request::Message::new()),
            MessageType::GPDU => 
                Message::GPDU(g_pdu::Message::new()),
        }
    }

    pub fn parse(message_type: MessageType, buffer: &[u8]) -> Option<(Self, usize)> {
        match message_type {
            MessageType::EchoRequest => {
                if let Some((m, pos)) = echo_request::Message::parse(buffer) {
                    Some(
                        (   
                            Message::EchoRequest(m),
                            pos
                        )
                    )
                }
                else {
                    None
                }
            },
            MessageType::EchoResponse => {
                if let Some((m, pos)) = echo_response::Message::parse(buffer) {
                    Some(
                        (   
                            Message::EchoResponse(m),
                            pos
                        )
                    )
                }
                else {
                    None
                }
            },
            MessageType::CreatePDPContextRequest => {
                if let Some((m, pos)) = create_pdp_context_request::Message::parse(buffer) {
                    Some(
                        (   
                            Message::CreatePDPContextRequest(m),
                            pos
                        )
                    )
                }
                else {
                    None
                }
            },
            MessageType::GPDU => {
                if let Some((m, pos)) = g_pdu::Message::parse(buffer) {
                    Some(
                        (   
                            Message::GPDU(m),
                            pos
                        )
                    )
                }
                else {
                    None
                }
            }
        }
    }
}

impl MessageTraits for Message {
    fn message_type(&self) -> u8 {
        match self {
            Message::EchoRequest(m) => m.message_type(),
            Message::EchoResponse(m) => m.message_type(),
            Message::CreatePDPContextRequest(m) => m.message_type(),
            Message::GPDU(m)=> m.message_type(),
        }
    }

    fn length(&self) -> u16 {
        match self {
            Message::EchoRequest(m) => m.length(),
            Message::EchoResponse(m) => m.length(),
            Message::CreatePDPContextRequest(m) => m.length(),
            Message::GPDU(m)=> m.length(),
        }
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        match self {
            Message::EchoRequest(m) => m.generate(buffer),
            Message::EchoResponse(m) => m.generate(buffer),
            Message::CreatePDPContextRequest(m) => m.generate(buffer),
            Message::GPDU(m)=> m.generate(buffer),
        }
    }

    fn push_ie(&mut self, ie: information_elements::InformationElement) {
        match self {
            Message::EchoRequest(m) => m.push_ie(ie),
            Message::EchoResponse(m) =>  m.push_ie(ie),
            Message::CreatePDPContextRequest(m) =>  m.push_ie(ie),
            Message::GPDU(m)=> m.push_ie(ie),
        }
    }
    fn pop_ie(&mut self) -> Option<information_elements::InformationElement>{
        match self {
            Message::EchoRequest(m) => m.pop_ie(),
            Message::EchoResponse(m) =>  m.pop_ie(),
            Message::CreatePDPContextRequest(m) =>  m.pop_ie(),
            Message::GPDU(m)=> m.pop_ie(),
        }
    }
}