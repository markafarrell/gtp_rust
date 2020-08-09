pub mod header;
pub mod messages;

use std::net::ToSocketAddrs;
use messages::{echo_request,echo_response,MessageType};

use crate::MTU;

pub struct Packet {
    pub header: header::Header,
    pub message: Box<dyn messages::MessageTraits>
}

impl Packet {
    pub fn new(message_type: MessageType) -> Self {
        Packet {
            header: header::Header::new(message_type),
            message: { 
                match message_type {
                    MessageType::EchoRequest => 
                        Box::new(echo_request::Message::new()),
                    MessageType::EchoResponse => 
                        Box::new(echo_response::Message::new())
                }
            }
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let end = self.header.generate(buffer);
        let end = end + self.message.generate(&mut buffer[end..]);

        end
    }
    pub fn send_to<A: ToSocketAddrs>(&self, socket: &std::net::UdpSocket, addr: A) -> std::io::Result<usize> {        
        let mut buffer = [0; MTU];

        let end = self.generate(&mut buffer);

        socket.send_to(&buffer[..end], addr)
    }
    pub fn parse(&mut self, _buffer: &[u8]) {
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;

    use crate::MTU;

    use messages::MessageType;
    use header::extension_headers::{
        mbms_support_indication,
        pdcp_pdu_number,
        suspend_request
    };

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let p = Packet::new(MessageType::EchoRequest);

        assert_eq!(p.header.message_type() as u8, MessageType::EchoRequest as u8);

        let end = p.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);
        
        buffer = [0; MTU];

        let p = Packet::new(MessageType::EchoResponse);

        assert_eq!(p.header.message_type() as u8, MessageType::EchoResponse as u8);

        let end = p.generate(&mut buffer);

        assert_eq!(buffer[..end], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoResponse as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);
    }

    #[test]
    fn test_send() {
        let socket = UdpSocket::bind("0.0.0.0:2123").expect("couldn't bind to address");

        let p = Packet::new(MessageType::EchoRequest);

        p.send_to(&socket, "192.168.1.1:2123").expect("Couldn't send data.");

        let mut p = Packet::new(MessageType::EchoResponse);

        p.header.set_teid(0x12345678);

        p.header.set_sequence_number(0x1234);
        p.header.enable_sequence_number();

        p.header.set_n_pdu_number(0x12);
        p.header.enable_n_pdu_number();
        
        let mbms_si = Box::new(mbms_support_indication::ExtensionHeader::new());
        p.header.push_extension_header(mbms_si);

        let mut pdcp_pdu_number = Box::new(pdcp_pdu_number::ExtensionHeader::new());
        pdcp_pdu_number.set_pdcp_pdu_number(5678);
        p.header.push_extension_header(pdcp_pdu_number);
        
        let s_req = Box::new(suspend_request::ExtensionHeader::new());
        p.header.push_extension_header(s_req);

        p.send_to(&socket, "192.168.1.1:2123").expect("Couldn't send data.");
    }
}