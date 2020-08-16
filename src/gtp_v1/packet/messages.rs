pub mod echo_request;
pub mod echo_response;
pub mod create_pdp_context_request;
pub mod information_elements;
pub mod g_pdu;

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
    fn push_ie(&mut self, ie: Box<dyn information_elements::InformationElementTraits>);
    fn pop_ie(&mut self) -> Option<Box<dyn information_elements::InformationElementTraits>>;
    fn attach_packet(&mut self, packet: &[u8]) -> Result<usize,String>;
}
