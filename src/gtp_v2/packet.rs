pub mod header;
pub mod messages;

use std::net::ToSocketAddrs;

use messages::{
    MessageTraits,
    Message,
};

use crate::MTU;

pub struct Packet {
    pub header: header::Header,
    pub message: Message,
}

impl Packet {
    pub fn new(message: Message) -> Self {
        Packet {
            header: header::Header::new(message.message_type()),
            message: message
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

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let h = header::Header::parse(&buffer);

        if let Some((mut h, h_pos)) = h {
            let m = messages::Message::parse(h.message_type(), &buffer[h_pos..]);

            if let Some((m, m_pos)) = m {
                h.set_payload_length(m.length());
                Some(
                    (
                        Packet {
                            header: h,
                            message: m
                        },
                        h_pos + m_pos
                    )
                )
            }
            else {
                None
            }
            
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::net::{
        UdpSocket,
    };

    use crate::MTU;

    use ascii::AsciiString;
    use std::net::{Ipv6Addr, Ipv4Addr};
    use messages::{MessageType, echo_response, echo_request, create_session_request, create_session_response};

    use messages::information_elements::{
        InformationElementType,
        recovery,
        rat_type,
        bearer_context,
        bearer_qos,
        ebi,
        f_teid,
        apn,
        imsi,
        pdn_type,
        pdn_address_allocation,
        msisdn,
        mei,
        serving_network,
        selection_mode,
        apn_restriction,
        ambr,
        ue_time_zone,
        charging_characteristics,
        cause,
    };

    use crate::gtp_v2::packet::messages::information_elements::user_location_information::{self, PLMN, CGI, SAI, RAI, TAI, ECGI, LAI, MeNBID, EMeNBID};

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let mut p = Packet::new(
            Message::EchoRequest(
                echo_request::Message::new(
                    recovery::InformationElement::new(0xAB,0).unwrap()
                )
            )
        );

        assert_eq!(p.header.message_type() as u8, MessageType::EchoRequest as u8);

        let pos = p.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0100_0000, 
            /* Message Type */ MessageType::EchoRequest as u8,
            /* Length */ 0, 9,
            /* Sequence Number */ 0x00, 0x00, 0x00, 
            /* Spare */ 0x00,
            InformationElementType::Recovery as u8,
            0, 1, 
            0, 
            0xAB
            ]
        );
        
        buffer = [0; MTU];

        let mut p = Packet::new(
            Message::EchoResponse(
                echo_response::Message::new(
                    recovery::InformationElement::new(0xCD,0).unwrap()
                )
            )
        );
        
        assert_eq!(p.header.message_type() as u8, MessageType::EchoResponse as u8);

        let pos = p.generate(&mut buffer);

        assert_eq!(buffer[..pos], [
            /* Flags */ 0b0100_0000, 
            /* Message Type */ MessageType::EchoResponse as u8,
            /* Length */ 0, 9,
            /* Sequence Number */ 0x00, 0x00, 0x00, 
            /* Spare */ 0x00,
            InformationElementType::Recovery as u8,
            0, 1, 
            0,
            0xCD
            ]
        );
    }

    #[test]
    fn test_send() {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

        let mut p = Packet::new(
            Message::EchoRequest(
                echo_request::Message::new(
                    recovery::InformationElement::new(0xAB,0).unwrap()
                )
            )
        );

        p.send_to(&socket, "192.168.1.1:2123").expect("Couldn't send data.");

        let mut p = Packet::new(
            Message::EchoResponse(
                echo_response::Message::new(
                    recovery::InformationElement::new(0xCD,0).unwrap()
                )
            )
        );

        p.header.enable_teid();
        p.header.set_teid(0x12345678);

        if let Ok(_) = p.header.set_sequence_number(0x876543){
            p.send_to(&socket, "192.168.1.1:2123").expect("Couldn't send data.");
        }
        else {
            assert!(false);
        }

        let mut p = Packet::new(
            Message::CreateSessionRequest(
                {
                    let mut mc = create_session_request::Message::new(
                        rat_type::InformationElement::new(rat_type::RATType::UTRAN, 0).unwrap(),
                        f_teid::InformationElement::new(
                            f_teid::InterfaceType::S11MmeGtpC,
                            0x87654321,
                            Some(Ipv4Addr::new(10,0,0,1)), 
                            None,
                            0
                        ).unwrap(),
                        {
                            let mut bc = bearer_context::InformationElement::new(
                                ebi::InformationElement::new(7, 0).unwrap(), 
                                bearer_qos::InformationElement::new(
                                    false,
                                    9,
                                    true,
                                    7,
                                    1_000_000, 1_000_000,
                                    0, 0,
                                    0
                                ).unwrap(), 
                                0
                            ).unwrap();
                            bc.set_s1_u_enodeb_f_teid(0x12345678, Some(Ipv4Addr::new(10,0,1,3)), None);
                            bc
                        },
                        apn::InformationElement::new(AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap(), 0).unwrap()
                    );
                    mc.imsi = Some(imsi::InformationElement::new("505013485090404", 0).unwrap());
                    mc.msisdn = Some(msisdn::InformationElement::new("61123456789", 0).unwrap());
                    mc.pdn_type = Some(pdn_type::InformationElement::new(pdn_type::PDNType::IPv4, 0).unwrap());
                    mc.pdn_address_allocation = Some(pdn_address_allocation::InformationElement::new(
                        pdn_type::PDNType::IPv6,
                        None,
                        Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)),
                        0
                    ).unwrap());
                    mc.uli = Some(
                        user_location_information::InformationElement::new(
                            Some(CGI::new(
                                PLMN::new([5,0,5], [0,9,9]),
                                0x1234,
                                0x4321,
                            )),
                            Some(SAI::new(
                                PLMN::new([5,0,5], [0,9,9]),
                                0x1234,
                                0x4321,
                            )),
                            Some(RAI::new(
                                PLMN::new([5,0,5], [0,9,9]),
                                0x1234,
                                0x4321,
                            )),
                            Some(TAI::new(
                                PLMN::new([5,0,5], [0,9,9]),
                                0x1234,
                            )),
                            Some(ECGI::new(
                                PLMN::new([5,0,5], [0,9,9]),
                                0xF_FF_FF_FF
                            ).unwrap()),
                            Some(LAI::new(
                                PLMN::new([5,0,5], [0,9,9]),
                                0x1234,
                            )),
                            Some(MeNBID::new(
                                PLMN::new([5,0,5], [0,9,9]),
                                0xFFFFF,
                            ).unwrap()),
                            Some(EMeNBID::new(
                                PLMN::new([5,0,5], [0,9,9]),
                                0x1FFFFF,
                            ).unwrap()),
                            0
                        ).unwrap()
                    );

                    mc.mei = Some(mei::InformationElement::new([1,2,3,4,5,6,7,8,9,1,2,3,4,5,6], Some(1), 0).unwrap());

                    mc.serving_network = Some(serving_network::InformationElement::new(PLMN::new([5,0,5], [0,9,9]), 0).unwrap());

                    mc.selection_mode = Some(selection_mode::InformationElement::new(selection_mode::SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified, 0).unwrap());

                    mc.maximum_apn_restriction = Some(apn_restriction::InformationElement::new(apn_restriction::MaximumAPNRestrictionValue::Unrestricted, 0).unwrap());

                    mc.apn_ambr = Some(ambr::InformationElement::new(0x12345678, 0x87654321, 0).unwrap());

                    mc.ue_time_zone = Some(ue_time_zone::InformationElement::new(0x40, ue_time_zone::DaylightSavingsTimeAdjustment::OneHourAdjustment, 0).unwrap()); // This is +10
                    // mc.ue_time_zone = Some(ue_time_zone::InformationElement::new(0b1100_0000, ue_time_zone::DaylightSavingsTimeAdjustment::OneHourAdjustment, 0).unwrap()); // This is -10

                    mc.charging_characteristics = Some(charging_characteristics::InformationElement::new(0x1234, 0).unwrap());

                    mc
                }
            )
        );

        p.send_to(&socket, "192.168.1.1:2123").expect("Couldn't send data.");

        let mut p = Packet::new(
            Message::CreateSessionResponse({
                let mut m = create_session_response::Message::new(
                    cause::InformationElement::new(
                        cause::CauseCode::RequestAccepted,
                        cause::CauseSource::LocalNode,
                        false,
                        false,
                        None,
                        0
                    ).unwrap(),
                    vec![
                        bearer_context::InformationElement::new(
                            ebi::InformationElement::new(7,0).unwrap(),
                            bearer_qos::InformationElement::new(
                                false,
                                9,
                                true,
                                7,
                                10_000_000,
                                10_000_000,
                                0,
                                0,
                                0
                            ).unwrap(),
                            0
                        ).unwrap()
                    ]
                );

                m.set_sender_f_teid_for_control_plane(0x87654321, Some(Ipv4Addr::new(10,0,0,2)), None);
                m.set_pgw_s5_s8_for_control_plane(0x12345678, Some(Ipv4Addr::new(10,0,0,3)), None);

                m.pdn_address_allocation = Some(
                    pdn_address_allocation::InformationElement::new(
                        pdn_type::PDNType::IPv4, 
                        Some(Ipv4Addr::new(10,0,0,1)), 
                        None, 
                        0
                    ).unwrap()
                );

                m.apn_restriction = Some(apn_restriction::InformationElement::new(apn_restriction::MaximumAPNRestrictionValue::Private2, 0).unwrap());

                m.apn_ambr = Some(ambr::InformationElement::new(0x12_34_56_78, 0x87_65_43_21, 0).unwrap());
                m
            })
        );

        p.send_to(&socket, "192.168.1.1:2123").expect("Couldn't send data.");
    }
}