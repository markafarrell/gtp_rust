pub mod echo_request;
pub mod echo_response;

#[derive(Copy, Clone)]
pub enum MessageType
{
    EchoRequest = 1,
    EchoResponse = 2
}

pub trait MessageTraits {
    fn message_type(&self) -> u8;
    fn length(&self) -> u8;
    fn generate(&self, buffer: &mut[u8]) -> usize;
    fn parse(&mut self, buffer: &[u8]);
}
