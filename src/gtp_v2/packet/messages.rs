pub mod echo_request;
pub mod echo_response;
pub mod create_session_request;
pub mod create_session_response;
pub mod information_elements;

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MessageType
{
    EchoRequest = 1,
    EchoResponse = 2,
    CreateSessionRequest = 32,
    CreateSessionResponse = 33,
}

impl TryFrom<u8> for MessageType
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MessageType::EchoRequest),
            2 => Ok(MessageType::EchoResponse),
            32 => Ok(MessageType::CreateSessionRequest),
            33 => Ok(MessageType::CreateSessionRequest),
            _ => Err(format!("Unsupported Message type ({})", value))
        }
    }
}

pub trait MessageTraits {
    fn message_type(&self) -> MessageType;
    fn length(&self) -> u16;
    fn generate(&self, buffer: &mut[u8]) -> usize;
}

pub enum Message {
    EchoRequest(echo_request::Message),
    EchoResponse(echo_response::Message),
    CreateSessionRequest(create_session_request::Message),
    CreateSessionResponse(create_session_response::Message),
}

impl Message {
    pub fn parse(message_type: MessageType, buffer: &[u8]) -> Option<(Self, usize)> {
        match message_type {
            MessageType::EchoRequest => {
                if let Some((m, pos)) = echo_request::Message::parse(buffer) {
                    Some((Message::EchoRequest(m), pos))
                } else { None }
            },
            MessageType::EchoResponse => {
                if let Some((m, pos)) = echo_response::Message::parse(buffer) {
                    Some((Message::EchoResponse(m), pos))
                } else { None }
            },
            MessageType::CreateSessionRequest => {
                if let Some((m, pos)) = create_session_request::Message::parse(buffer) {
                    Some((Message::CreateSessionRequest(m), pos))
                } else { None }
            },
            MessageType::CreateSessionResponse => {
                if let Some((m, pos)) = create_session_response::Message::parse(buffer) {
                    Some((Message::CreateSessionResponse(m), pos))
                } else { None }
            },
        }
    }
}

impl MessageTraits for Message {
    fn message_type(&self) -> MessageType {
        match self {
            Message::EchoRequest(m) => m.message_type(),
            Message::EchoResponse(m) => m.message_type(),
            Message::CreateSessionRequest(m) => m.message_type(),
            Message::CreateSessionResponse(m) => m.message_type(),
        }
    }

    fn length(&self) -> u16 {
        match self {
            Message::EchoRequest(m) => m.length(),
            Message::EchoResponse(m) => m.length(),
            Message::CreateSessionRequest(m) => m.length(),
            Message::CreateSessionResponse(m) => m.length(),
        }
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        match self {
            Message::EchoRequest(m) => m.generate(buffer),
            Message::EchoResponse(m) => m.generate(buffer),
            Message::CreateSessionRequest(m) => m.generate(buffer),
            Message::CreateSessionResponse(m) => m.generate(buffer),
        }
    }
}
