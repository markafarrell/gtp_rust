use super::{MessageTraits, MessageType};

use super::information_elements::{
    InformationElement,
    InformationElementTraits,
    cause,
    bearer_context,
    f_teid,
    pdn_address_allocation,
    apn_restriction,
    ambr,
};

use std::net::{Ipv4Addr, Ipv6Addr};

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug)]
enum FTeidInstance {
    SenderFTeidForControlPlane = 0,
    PgwS5S8AddressForControlPlane = 1
}

impl TryFrom<u8> for FTeidInstance
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FTeidInstance::SenderFTeidForControlPlane),
            1 => Ok(FTeidInstance::PgwS5S8AddressForControlPlane),
            _ => Err(format!("Unsupported F-TEID Instance ({})", value))
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum BearerContextInstance {
    ToBeCreated = 0,
    ToBeRemoved = 1
}

impl TryFrom<u8> for BearerContextInstance
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BearerContextInstance::ToBeCreated),
            1 => Ok(BearerContextInstance::ToBeRemoved),
            _ => Err(format!("Unsupported Bearer Context Instance Instance ({})", value)),
        }
    }
}

pub struct Message {
    /*
        --------------------------------------------|---------------------------|-------------------------                             
        Information Element                         |   Presence requirement    |   Reference 
        --------------------------------------------|---------------------------|-------------------------
        Cause                                       |    Mandatory              |    
        Change Reporting Action                     |    Conditional            |   
        CSG Information Reporting Action            |    Conditional Optional   |
        H(e)NB Information Reporting                |    Conditional Optional   |
        Sender F-TEID for Control Plane             |    Conditional            |   8.22
        PGW S5/S8 Address for Control Plane or PMIP |    Conditional            |
        PDN Address Allocation                      |    Conditional            |   8.14
        APN Restriction                             |    Conditional            |
        Aggregate Maximum Bitrate (APN-AMBR)        |    Conditional            |
        Linked EPS Bearer ID                        |    Conditional            |
        Protocol Configuration Options (PCO)        |    Conditional            |
        Bearer Contexts created                     |    Mandatory              |   8.28
        Bearer Contexts marked for removal          |    Conditional            |
        Recovery                                    |    Conditional            |
        Charging Gateway Name                       |    Conditional            |
        Charging Gateway Address                    |    Conditional            |
        PGW-FQ-CSID                                 |    Conditional            |
        SGW-FQ-CSID                                 |    Conditional            |
        SGW LDN                                     |    Optional               |
        PGW LDN                                     |    Optional               |
        PGW Back-Off Time                           |    Optional               |
        Additional Protocol Configuration Options   |    Conditional Optional   |
        Trusted WLAN IPv4 Parameters                |    Conditional Optional   |
        Indication Flags                            |    Conditional Optional   |
        Presence Reporting Area Action              |    Conditional Optional   |
        PGW's Node Level Load Control Information   |    Optional               |
        PGW's APN Level Load Control Information    |    Optional               |
        SGW's Node Level Load Control Information   |    Optional               |
        PGW Overload Control Information            |    Optional               |
        SGW Overload Control Information            |    Optional               |
        NBIFOM Container                            |    Conditional Optional   |
        PDN Connection Charging ID                  |    Conditional Optional   |
        Extended Protocol Configuration Options     |    Conditional Optional   |
        Private Extension                           |    Optional               |
        --------------------------------------------|---------------------------|-------------------------
    */

    pub cause: cause::InformationElement,
    pub sender_f_teid_for_control_plane: Option<f_teid::InformationElement>,
    pub pgw_s5_s8_for_control_plane: Option<f_teid::InformationElement>,
    pub pdn_address_allocation: Option<pdn_address_allocation::InformationElement>,
    pub apn_restriction: Option<apn_restriction::InformationElement>,
    pub apn_ambr: Option<ambr::InformationElement>,
    pub bearer_contexts_created: Vec<bearer_context::InformationElement>,
    pub bearer_contexts_marked_for_removal: Vec<bearer_context::InformationElement>,
}

impl Message {
    pub fn new(
        cause: cause::InformationElement,
        bearer_contexts_created: Vec<bearer_context::InformationElement>,
    ) -> Message {
        let mut m = 
            Message {
                cause,
                sender_f_teid_for_control_plane: None,
                pgw_s5_s8_for_control_plane: None,
                pdn_address_allocation: None,
                apn_restriction: None,
                apn_ambr: None,
                bearer_contexts_created: Vec::new(),
                bearer_contexts_marked_for_removal: Vec::new(),
            };

        for bc in bearer_contexts_created {
            m.push_bearer_context_created(bc);
        }

        m
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        // Keep trying to parse IEs until the end of the buffer
        let mut pos = 0;

        let mut cause: Option<cause::InformationElement> = None;
        let mut sender_f_teid_for_control_plane: Option<f_teid::InformationElement> = None;
        let mut pgw_s5_s8_for_control_plane: Option<f_teid::InformationElement> = None;
        let mut pdn_address_allocation: Option<pdn_address_allocation::InformationElement> = None;
        let mut apn_restriction: Option<apn_restriction::InformationElement> = None;
        let mut apn_ambr: Option<ambr::InformationElement> = None;
        let mut bearer_contexts_created: Vec<bearer_context::InformationElement> = Vec::new();
        let mut bearer_contexts_marked_for_removal: Vec<bearer_context::InformationElement> = Vec::new();

        while pos < buffer.len()
        {
            if let Some((ie, ie_pos)) = InformationElement::parse(&buffer[pos..]){
                match ie {
                    InformationElement::BearerContext(ie) => {
                        if let Ok(instance) = BearerContextInstance::try_from(ie.instance()) {
                            match instance
                            {
                                BearerContextInstance::ToBeCreated => bearer_contexts_created.push(ie),
                                BearerContextInstance::ToBeRemoved => bearer_contexts_marked_for_removal.push(ie)
                            }
                        }
                        else { /* Not an instance of Bearer context that we expect. Just ignore it */ }
                    },
                    InformationElement::FTEID(ie) => {
                        if let Ok(instance) = FTeidInstance::try_from(ie.instance()) {
                            match instance
                            {
                                FTeidInstance::SenderFTeidForControlPlane => sender_f_teid_for_control_plane = Some(ie),
                                FTeidInstance::PgwS5S8AddressForControlPlane => pgw_s5_s8_for_control_plane = Some(ie)
                            }
                        }
                        else { /* Not an instance of F-TEID that we expect. Just ignore it */ }
                    },
                    InformationElement::PDNAddressAllocation(ie) => pdn_address_allocation = Some(ie),
                    InformationElement::APNRestriction(ie) => apn_restriction = Some(ie),
                    InformationElement::AMBR(ie) => apn_ambr = Some(ie),
                    InformationElement::Cause(ie) => cause = Some(ie),
                    _ =>  { /* Its an IE that we didn't expect. Just ignore it */ }
                }
                pos = pos + ie_pos;
            }
            else {
                // IE parsing failed
                pos = pos + InformationElement::skip_parsing(&buffer[pos..]);
            }
        }

        if cause.is_some() && 
            bearer_contexts_created.len() > 1 {
                Some((
                    Message {
                        cause: cause.unwrap(),
                        sender_f_teid_for_control_plane,
                        pgw_s5_s8_for_control_plane,
                        pdn_address_allocation,
                        apn_restriction,
                        apn_ambr,
                        bearer_contexts_created,
                        bearer_contexts_marked_for_removal,
                    }, 
                    pos
                ))
        }
        else { None }
    }

    pub fn set_pgw_s5_s8_for_control_plane(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.pgw_s5_s8_for_control_plane = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S5S8PgwGtpC,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::PgwS5S8AddressForControlPlane as u8
        ).unwrap());
    }

    pub fn unset_pgw_s5_s8_for_control_plane(&mut self) {
        self.pgw_s5_s8_for_control_plane = None;
    }

    pub fn set_sender_f_teid_for_control_plane(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.sender_f_teid_for_control_plane = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S11S4SgwGtpC,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::SenderFTeidForControlPlane as u8
        ).unwrap());
    }

    pub fn unset_sender_f_teid_for_control_plane(&mut self) {
        self.sender_f_teid_for_control_plane = None;
    }

    pub fn push_bearer_context_created(&mut self, mut bearer_context: bearer_context::InformationElement) {
        bearer_context.set_instance(BearerContextInstance::ToBeCreated as u8).unwrap();
        self.bearer_contexts_created.push(bearer_context);
    }

    pub fn push_bearer_context_marked_for_removal(&mut self, mut bearer_context: bearer_context::InformationElement) {
        bearer_context.set_instance(BearerContextInstance::ToBeRemoved as u8).unwrap();
        self.bearer_contexts_marked_for_removal.push(bearer_context);
    }
}

impl MessageTraits for Message {
    fn message_type(&self) -> MessageType {
        MessageType::CreateSessionResponse
    }

    fn length(&self) -> u16 {
        let mut length = 0;

        length = length + self.cause.length();

        if let Some(ref ie) = self.pgw_s5_s8_for_control_plane {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.pgw_s5_s8_for_control_plane {
            length = length + ie.length();
        }

        for ie in &self.bearer_contexts_created {
            length = length + ie.length();
        }

        for ie in &self.bearer_contexts_marked_for_removal {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.pdn_address_allocation {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.apn_restriction {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.apn_ambr {
            length = length + ie.length();
        }

        length
    }
    fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = 0;

        pos = pos + self.cause.generate(&mut buffer[pos..]);

        if let Some(ref ie) = self.sender_f_teid_for_control_plane {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.pgw_s5_s8_for_control_plane {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        for ie in &self.bearer_contexts_created {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        for ie in &self.bearer_contexts_marked_for_removal {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.pdn_address_allocation {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.apn_restriction {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.apn_ambr {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        pos
    }
}

#[cfg(test)]
mod tests {
    extern crate ascii;

    use super::*;
    use crate::MTU;
    use crate::gtp_v2::packet::messages::MessageTraits;
    use crate::gtp_v2::packet::messages::MessageType;
    use crate::gtp_v2::packet::messages::information_elements::{InformationElementType,
        bearer_context,
        bearer_qos,
        ebi,
        f_teid,
        pdn_type,
        pdn_address_allocation,
        apn_restriction,
        ambr,
        cause,
    };
    
    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let mut m = Message::new(
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

        let expected = [
            InformationElementType::Cause as u8,
            0, 2, // Length
            0, // Spare
            cause::CauseCode::RequestAccepted as u8,
            0b00000000,
            InformationElementType::FTEID as u8,
            0, 9, // Length
            FTeidInstance::SenderFTeidForControlPlane as u8, // Spare and Instance
            (0b1 << 7) | (0b0 << 6) | (f_teid::InterfaceType::S11S4SgwGtpC as u8),
            0x87, 0x65, 0x43, 0x21,
            10, 0, 0, 2,
            InformationElementType::FTEID as u8,
            0, 9, // Length
            FTeidInstance::PgwS5S8AddressForControlPlane as u8, // Spare and Instance
            (0b1 << 7) | (0b0 << 6) | (f_teid::InterfaceType::S5S8PgwGtpC as u8),
            0x12, 0x34, 0x56, 0x78,
            10, 0, 0, 3,
            InformationElementType::BearerContext as u8,
            0, 31, // Length
            0, // Spare
            InformationElementType::EBI as u8,
            0, 1, // Length
            0, // Spare
            7, // EPS Bearer ID
            InformationElementType::BearerQoS as u8,
            0, 22, // Length
            0, // Spare and Instance
            0b01100100, // Flags
            7, // QCI
            0x00, 0x00, 0x98, 0x96, 0x80,
            0x00, 0x00, 0x98, 0x96, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
            InformationElementType::PDNAddressAllocation as u8,
            0, 5, // Length
            0, // Spare
            pdn_type::PDNType::IPv4 as u8, // PDN Type
            10,0,0,1,
            InformationElementType::APNRestriction as u8,
            0, 1, // Length
            0, // Spare
            4, // APN Restriction
            InformationElementType::AMBR as u8,
            0, 8, // Length
            0, // Spare
            0x12, 0x34, 0x56, 0x78, // AMBR for uplink
            0x87, 0x65, 0x43, 0x21, // AMBR for uplink
        ];

        let pos = m.generate(&mut buffer);

        for i in 0..pos {
            if buffer[i] != expected[i] {
                println!("{} (actual) != {} (expected) at byte {}", buffer[i], expected[i], i);
                assert!(false);
            } 
        }
    }
    
    #[test]
    fn test_length() {
        let mut m = Message::new(
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

        assert_eq!(m.length(), 93)
    }

    #[test]
    fn test_message_type() {
        let mut m = Message::new(
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

        assert_eq!(m.message_type(), MessageType::CreateSessionResponse)
    }

    #[test]
    fn test_message_parse() {
        let message_bytes = [
            InformationElementType::Cause as u8,
            0, 2, // Length
            0, // Spare
            cause::CauseCode::RequestAccepted as u8,
            0b00000000,
            InformationElementType::FTEID as u8,
            0, 9, // Length
            0, // Spare and Instance
            (0b1 << 7) | (0b0 << 6) | (f_teid::InterfaceType::S11S4SgwGtpC as u8),
            0x87, 0x65, 0x43, 0x21,
            10, 0, 0, 2,
            InformationElementType::FTEID as u8,
            0, 9, // Length
            0, // Spare and Instance
            (0b1 << 7) | (0b0 << 6) | (f_teid::InterfaceType::S5S8PgwGtpC as u8),
            0x12, 0x34, 0x56, 0x78,
            10, 0, 0, 3,
            InformationElementType::BearerContext as u8,
            0, 31, // Length
            0, // Spare
            InformationElementType::EBI as u8,
            0, 1, // Length
            0, // Spare
            7, // EPS Bearer ID
            InformationElementType::BearerQoS as u8,
            0, 22, // Length
            0, // Spare and Instance
            0b01100100, // Flags
            7, // QCI
            0x00, 0x00, 0x98, 0x96, 0x80,
            0x00, 0x00, 0x98, 0x96, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
            InformationElementType::PDNAddressAllocation as u8,
            0, 5, // Length
            0, // Spare
            pdn_type::PDNType::IPv4 as u8, // PDN Type
            10,0,0,1,
            InformationElementType::APNRestriction as u8,
            0, 1, // Length
            0, // Spare
            4, // APN Restriction
            InformationElementType::AMBR as u8,
            0, 8, // Length
            0, // Spare
            0x12, 0x34, 0x56, 0x78, // AMBR for uplink
            0x87, 0x65, 0x43, 0x21, // AMBR for uplink
        ];

        if let Some((m, _pos)) = Message::parse(&message_bytes){

            

            if let Some(ie) = m.sender_f_teid_for_control_plane {
                if let Some(a) = ie.ipv4_address {
                    assert_eq!(a, Ipv4Addr::new(10,0,0,2));
                }
                else {
                    assert!(false);
                }
                assert_eq!(ie.teid, 0x87654321);
                assert_eq!(ie.interface_type, f_teid::InterfaceType::S11S4SgwGtpC);
            }
            else {
                assert!(false);
            }

            if let Some(ie) = m.pgw_s5_s8_for_control_plane {
                if let Some(a) = ie.ipv4_address {
                    assert_eq!(a, Ipv4Addr::new(10,0,0,3));
                }
                else {
                    assert!(false);
                }
                assert_eq!(ie.teid, 0x87654321);
                assert_eq!(ie.interface_type, f_teid::InterfaceType::S5S8PgwGtpC);
            }
            else {
                assert!(false);
            }
            
            assert_eq!(m.bearer_contexts_created[0].eps_bearer_id.eps_bearer_id, 7);
            assert_eq!(m.bearer_contexts_created[0].bearer_level_qos.pvi, true);
            assert_eq!(m.bearer_contexts_created[0].bearer_level_qos.pl(), 9);
            assert_eq!(m.bearer_contexts_created[0].bearer_level_qos.pci, false);
            assert_eq!(m.bearer_contexts_created[0].bearer_level_qos.qci, 7);
            assert_eq!(m.bearer_contexts_created[0].bearer_level_qos.guaranteed_ul_bitrate(), 0);
            assert_eq!(m.bearer_contexts_created[0].bearer_level_qos.guaranteed_dl_bitrate(), 0);
            assert_eq!(m.bearer_contexts_created[0].bearer_level_qos.max_ul_bitrate(), 10_000_000);
            assert_eq!(m.bearer_contexts_created[0].bearer_level_qos.max_dl_bitrate(), 10_000_000);

            if let Some(pdn_address_allocation) = m.pdn_address_allocation {
                assert_eq!(pdn_address_allocation.pdn_type, pdn_type::PDNType::IPv4);
                assert_eq!(pdn_address_allocation.ipv4_address, Some(Ipv4Addr::new(10,0,0,1)));
            }
            else { assert!(false); }

            if let Some(ie) = m.apn_restriction {
                assert_eq!(ie.maximum_apn_restriction, apn_restriction::MaximumAPNRestrictionValue::Private2);
            }
            else { assert!(false); }

            if let Some(ie) = m.apn_ambr {
                assert_eq!(ie.uplink, 0x12_34_56_78);
                assert_eq!(ie.uplink, 0x87_65_43_21);
            }
            else { assert!(false); }
        }
    }
}
