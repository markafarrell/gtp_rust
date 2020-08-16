pub mod header;
pub mod messages;

use std::net::ToSocketAddrs;
use messages::{
    echo_request,
    echo_response,
    create_pdp_context_request,
    g_pdu,
    MessageType,
    MessageTraits
};

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
                        Box::new(echo_response::Message::new()),
                    MessageType::CreatePDPContextRequest => 
                        Box::new(create_pdp_context_request::Message::new()),
                    MessageType::GPDU => 
                        Box::new(g_pdu::Message::new())
                }
            }
        }
    }

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let h = header::Header::parse(&buffer);

        if let Some((mut h, length, pos)) = h {
            match h.message_type() {
                MessageType::EchoRequest => {
                    if let Some((m, pos)) = echo_request::Message::parse(&buffer[pos..]) {
                        h.set_payload_length(m.length());
                        Some(
                            (   
                                Packet {
                                    header: h,
                                    message: Box::new(m)
                                },
                                pos
                            )
                        )
                    }
                    else {
                        None
                    }
                },
                MessageType::EchoResponse => {
                    if let Some((m, pos)) = echo_response::Message::parse(&buffer[pos..]) {
                        h.set_payload_length(m.length());
                        Some(
                            (   
                                Packet {
                                    header: h,
                                    message: Box::new(m)
                                },
                                pos
                            )
                        )
                    }
                    else {
                        None
                    }
                },
                MessageType::CreatePDPContextRequest => {
                    if let Some((m, pos)) = echo_response::Message::parse(&buffer[pos..]) {
                        h.set_payload_length(m.length());
                        Some(
                            (   
                                Packet {
                                    header: h,
                                    message: Box::new(m)
                                },
                                pos
                            )
                        )
                    }
                    else {
                        None
                    }
                },
                MessageType::GPDU => {
                    if let Some((m, pos)) = g_pdu::Message::parse(&buffer[pos..]) {
                        h.set_payload_length(m.length());
                        Some(
                            (   
                                Packet {
                                    header: h,
                                    message: Box::new(m)
                                },
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
        else {
            None
        }
    }

    pub fn generate(&mut self, buffer: &mut[u8]) -> usize {
        self.header.set_payload_length(self.message.length());

        let pos = self.header.generate(buffer);
        let pos = pos + self.message.generate(&mut buffer[pos..]);

        pos
    }
    pub fn send_to<A: ToSocketAddrs>(&mut self, socket: &std::net::UdpSocket, addr: A) -> std::io::Result<usize> {        
        let mut buffer = [0; MTU];

        let pos = self.generate(&mut buffer);

        socket.send_to(&buffer[..pos], addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::net::{
        UdpSocket,
        IpAddr,
        Ipv4Addr,
        Ipv6Addr
    };

    use crate::MTU;

    use messages::{
        MessageType,
        information_elements
    };

    use header::extension_headers::{
        mbms_support_indication,
        pdcp_pdu_number,
        suspend_request
    };

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let mut p = Packet::new(MessageType::EchoRequest);

        assert_eq!(p.header.message_type() as u8, MessageType::EchoRequest as u8);

        let pos = p.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);
        
        buffer = [0; MTU];

        let mut p = Packet::new(MessageType::EchoResponse);

        assert_eq!(p.header.message_type() as u8, MessageType::EchoResponse as u8);

        let pos = p.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::EchoResponse as u8,
            /* Length */ 0x00, 0x00,
            /* TEID */ 0x00, 0x00, 0x00, 0x00
            ]);
    }

    #[test]
    fn test_send() {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

        let mut p = Packet::new(MessageType::EchoRequest);

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

        let mut p = Packet::new(MessageType::CreatePDPContextRequest);
        
        p.message.push_ie(
            Box::new(
                information_elements::teid_data_i::InformationElement::new(0x12345678)
            )
        );

        let nsapi = information_elements::nsapi::InformationElement::new(0xF);

        if let Ok(nsapi) = nsapi {
            p.message.push_ie(
                Box::new(nsapi)
            );
        }
        
        p.message.push_ie(
            Box::new(
                information_elements::gsn_address::InformationElement::new(
                    IpAddr::V4(
                        Ipv4Addr::new(192,168,0,1)
                    )
                )
            )
        );
        
        p.message.push_ie(
            Box::new(
                information_elements::gsn_address::InformationElement::new(
                    IpAddr::V6(
                        Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE)
                    )
                )
            )
        );

        p.message.push_ie(
            Box::new(
                information_elements::qos_profile::InformationElement::new(
                    8,
                    information_elements::qos_profile::DelayClass::BestEffort,
                    information_elements::qos_profile::ReliabilityClass::UnAckGTPUnAckLLCUnAckRLCUnProtectedData,
                    information_elements::qos_profile::PeakThroughput::UpTo1000OctetsPerSecond,
                    information_elements::qos_profile::PrecedenceClass::NormalPriority,
                    information_elements::qos_profile::MeanThroughput::BestEffort,
                )
            )
        );

        p.send_to(&socket, "192.168.1.1:2123").expect("Couldn't send data.");

        let mut p = Packet::new(MessageType::GPDU);

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
        
        if let Ok(_) = p.message.attach_packet(&icmpv4)
        {
            p.send_to(&socket, "192.168.1.1:2152").expect("Couldn't send data.");
        }
    }
    #[test]
    fn test_parse() {
        let p_bytes =  [
            /* Flags */ 0b0011_0000, 
            /* Message Type */ MessageType::GPDU as u8,
            /* Length */ 0x00, 0x54,
            /* TEID */ 0x87, 0x65, 0x43, 0x21,
            /* icmp packeet */
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
        
        if let Some((p, pos)) = Packet::parse(&p_bytes) {
            assert_eq!(p.message.message_type() as u8, MessageType::GPDU as u8);
            assert_eq!(p.header.length(), 0x54);
        }
        else {
            assert!(false);
        }
    }
}