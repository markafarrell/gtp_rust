use super::{MessageTraits, MessageType};

use super::information_elements::{
    InformationElement,
    InformationElementTraits,
    apn,
    bearer_context,
    f_teid,
    rat_type,
    imsi,
    pdn_type,
    pdn_address_allocation,
    msisdn,
    user_location_information,
    mei,
    serving_network,
    selection_mode,
    apn_restriction,
    ambr,
    ue_time_zone,
    charging_characteristics,
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
        IMSI                                        |    Conditional            |   8.3  
        MSISDN                                      |    Conditional            |   8.11
        ME Identity (MEI)                           |    Conditional            |
        User Location Information (ULI)             |    Conditional            |
        Serving Network                             |    Conditional            |
        RAT Type                                    |    Mandatory              |   8.17
        Indication Flags                            |    Conditional            |
        Sender F-TEID for Control Plane             |    Mandatory              |   8.22
        PGW S5/S8 Address for Control Plane or PMIP |    Conditional            |
        Access Point Name                           |    Mandatory              |   8.6
        Selection Mode                              |    Conditional            |
        PDN Type                                    |    Conditional            |   8.34
        PDN Address Allocation                      |    Conditional            |   8.14
        Maxmimum APN Restriction                    |    Conditional            |
        Aggregate Maximum Bitrate (APN-AMBR)        |    Conditional            |
        Linked EPS Bearer ID                        |    Conditional            |
        Trusted WLAN Mode Indication                |    Conditional Optional   |
        Protocol Configuration Options (PCO)        |    Conditional            |
        Bearer Contexts to be created               |    Mandatory              |   8.28
        Bearer Contexts to be removed               |    Conditional            |
        Trace Information                           |    Conditional            |
        Recovery                                    |    Conditional            |
        MME-FQ-CSID                                 |    Conditional            |
        SGW-FQ-CSID                                 |    Conditional            |
        ePDG-FQ-CSID                                |    Conditional            |
        TWAN-FQ-CSID                                |    Conditional            |
        UE Time Zone                                |    Conditional            |
        User CSG Information (UCI)                  |    Conditional Optional   |
        Charging Characteristics                    |    Conditional            |
        MME/S4-SGSN LDN                             |    Optional               |
        SGW LDN                                     |    Optional               |
        ePDG LDN                                    |    Optional               |
        TWAN LDN                                    |    Optional               |
        Signalling Priority Indication              |    Conditional Optional   |
        UE Local IP Address                         |    Conditional Optional   |
        UE UDP Port                                 |    Conditional Optional   |
        Additional Protocol Configuration Options   |    Conditional Optional   |
        H(e)NB Local IP Address                     |    Conditional Optional   |
        H(e)NB UDP Port                             |    Conditional Optional   |
        MME/S4-SGSN Identifier                      |    Conditional Optional   |
        TWAN Identifier                             |    Conditional Optional   |
        ePDG IP Address                             |    Optional               |
        CN Operator Selection Entity                |    Conditional Optional   |
        Presence Reporting Area Information         |    Conditional Optional   |
        MME/S4-SGSN Overload Control Information    |    Optional               |
        SGW Overload Control Information            |    Optional               |
        TWAN/ePDG Overload Control Information      |    Optional               |
        Originating Time Stamp                      |    Conditional Optional   |
        Maximum Wait Time                           |    Conditional Optional   |
        WLAN Location Information                   |    Conditional Optional   |
        NBIFOM Container                            |    Conditional Optional   |
        Remote UE Context Connected                 |    Conditional Optional   |
        3GPP AAA Server Identifier                  |    Optional               |
        Extended Protocol Configuration Options     |    Conditional Optional   |
        Serving PLMN Rate Control                   |    Conditional Optional   |
        MO Exception Data Counter                   |    Conditional Optional   |
        UE TCP Port                                 |    Conditional Optional   |
        Mapped UE Usage Type                        |    Conditional Optional   |
        User Location Information for SGW           |    Conditional Optional   |
        SGW-U Node Name                             |    Conditional Optional   |
        Secondary RAT Usage Data Report             |    Conditional Optional   |
        UP Function Selection Indication Flags      |    Conditional Optional   |
        APN RATE Control Status                     |    Conditional Optional   |
        Private Extension                           |    Optional               |
        --------------------------------------------|---------------------------|-------------------------
    */

    pub imsi: Option<imsi::InformationElement>,
    pub msisdn: Option<msisdn::InformationElement>,
    pub mei: Option<mei::InformationElement>,
    pub uli: Option<user_location_information::InformationElement>,
    pub serving_network: Option<serving_network::InformationElement>,
    pub rat_type: rat_type::InformationElement,
    pub sender_f_teid_for_control_plane: f_teid::InformationElement,
    pub pgw_s5_s8_for_control_plane: Option<f_teid::InformationElement>,
    pub apn: apn::InformationElement,
    pub selection_mode: Option<selection_mode::InformationElement>,
    pub pdn_type: Option<pdn_type::InformationElement>,
    pub pdn_address_allocation: Option<pdn_address_allocation::InformationElement>,
    pub maximum_apn_restriction: Option<apn_restriction::InformationElement>,
    pub apn_ambr: Option<ambr::InformationElement>,
    pub bearer_contexts_to_be_created: Vec<bearer_context::InformationElement>,
    pub bearer_contexts_to_be_removed: Vec<bearer_context::InformationElement>,
    pub ue_time_zone: Option<ue_time_zone::InformationElement>,
    pub charging_characteristics: Option<charging_characteristics::InformationElement>,
}

impl Message {
    pub fn new(
        rat_type: rat_type::InformationElement,
        sender_f_teid_for_control_plane: f_teid::InformationElement,
        bearer_context_to_be_created: bearer_context::InformationElement,
        apn: apn::InformationElement,
    ) -> Message {
        let mut m = Message {
            rat_type,
            sender_f_teid_for_control_plane,
            pgw_s5_s8_for_control_plane: None,
            bearer_contexts_to_be_created: Vec::new(),
            bearer_contexts_to_be_removed: Vec::new(),
            apn,
            imsi: None,
            pdn_type: None,
            pdn_address_allocation: None,
            msisdn: None,
            uli: None,
            mei: None,
            serving_network: None,
            selection_mode: None,
            maximum_apn_restriction: None,
            apn_ambr: None,
            ue_time_zone: None,
            charging_characteristics: None,
        };

        m.push_bearer_context_to_be_created(bearer_context_to_be_created);

        m
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        // Keep trying to parse IEs until the end of the buffer
        let mut pos = 0;

        let mut imsi: Option<imsi::InformationElement> = None;
        let mut msisdn: Option<msisdn::InformationElement> = None;
        let mut mei: Option<mei::InformationElement> = None;
        let mut uli: Option<user_location_information::InformationElement> = None;
        let mut serving_network: Option<serving_network::InformationElement> = None;
        let mut rat_type: Option<rat_type::InformationElement> = None;
        let mut sender_f_teid_for_control_plane: Option<f_teid::InformationElement> = None;
        let mut pgw_s5_s8_for_control_plane: Option<f_teid::InformationElement> = None;
        let mut apn: Option<apn::InformationElement> = None;
        let mut selection_mode: Option<selection_mode::InformationElement> = None;
        let mut pdn_type: Option<pdn_type::InformationElement> = None;
        let mut pdn_address_allocation: Option<pdn_address_allocation::InformationElement> = None;
        let mut maximum_apn_restriction: Option<apn_restriction::InformationElement> = None;
        let mut apn_ambr: Option<ambr::InformationElement> = None;
        let mut bearer_contexts_to_be_created: Vec<bearer_context::InformationElement> = Vec::new();
        let mut bearer_contexts_to_be_removed: Vec<bearer_context::InformationElement> = Vec::new();
        let mut ue_time_zone: Option<ue_time_zone::InformationElement> = None;
        let mut charging_characteristics: Option<charging_characteristics::InformationElement> = None;

        while pos < buffer.len()
        {
            if let Some((ie, ie_pos)) = InformationElement::parse(&buffer[pos..]){
                match ie {
                    InformationElement::IMSI(ie) => imsi = Some(ie),
                    InformationElement::MSISDN(ie) => msisdn = Some(ie),
                    InformationElement::MEI(ie) => mei = Some(ie),
                    InformationElement::ULI(ie) => uli = Some(ie),
                    InformationElement::ServingNetwork(ie) => serving_network = Some(ie),
                    InformationElement::RATType(ie) => rat_type = Some(ie),
                    InformationElement::BearerContext(ie) => {
                        if let Ok(instance) = BearerContextInstance::try_from(ie.instance()) {
                            match instance
                            {
                                BearerContextInstance::ToBeCreated => bearer_contexts_to_be_created.push(ie),
                                BearerContextInstance::ToBeRemoved => bearer_contexts_to_be_removed.push(ie)
                            }
                        }
                        else { /* Not an instance of Bearer context that we expect. Just ignore it */ }
                    },
                    InformationElement::APN(ie) => apn = Some(ie),
                    InformationElement::UETimeZone(ie) => ue_time_zone = Some(ie),
                    InformationElement::SelectionMode(ie) => selection_mode = Some(ie),
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
                    InformationElement::PDNType(ie) => pdn_type = Some(ie),
                    InformationElement::PDNAddressAllocation(ie) => pdn_address_allocation = Some(ie),
                    InformationElement::APNRestriction(ie) => maximum_apn_restriction = Some(ie),
                    InformationElement::AMBR(ie) => apn_ambr = Some(ie),
                    InformationElement::ChargingCharacteristics(ie) => charging_characteristics = Some(ie),
                    _ =>  { /* Its an IE that we didn't expect. Just ignore it */ }
                }
                pos = pos + ie_pos;
            }
            else {
                // IE parsing failed
                pos = pos + InformationElement::skip_parsing(&buffer[pos..]);
            }
        }

        if rat_type.is_some() && 
            bearer_contexts_to_be_created.len() > 1 &&
            sender_f_teid_for_control_plane.is_some() &&
            apn.is_some() {
                Some((
                    Message {
                        rat_type: rat_type.unwrap(),
                        sender_f_teid_for_control_plane: sender_f_teid_for_control_plane.unwrap(),
                        bearer_contexts_to_be_created,
                        bearer_contexts_to_be_removed,
                        apn: apn.unwrap(),
                        pgw_s5_s8_for_control_plane,
                        imsi,
                        pdn_type,
                        pdn_address_allocation,
                        msisdn,
                        uli,
                        mei,
                        serving_network,
                        selection_mode,
                        maximum_apn_restriction,
                        apn_ambr,
                        ue_time_zone,
                        charging_characteristics,
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

    pub fn push_bearer_context_to_be_created(&mut self, mut bearer_context: bearer_context::InformationElement) {
        bearer_context.set_instance(BearerContextInstance::ToBeCreated as u8).unwrap();
        self.bearer_contexts_to_be_created.push(bearer_context);
    }

    pub fn push_bearer_context_to_be_removed(&mut self, mut bearer_context: bearer_context::InformationElement) {
        bearer_context.set_instance(BearerContextInstance::ToBeRemoved as u8).unwrap();
        self.bearer_contexts_to_be_removed.push(bearer_context);
    }
}

impl MessageTraits for Message {
    fn message_type(&self) -> MessageType {
        MessageType::CreateSessionRequest
    }

    fn length(&self) -> u16 {
        let mut length = 0;

        if let Some(ref ie) = self.imsi {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.msisdn {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.mei {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.uli {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.serving_network {
            length = length + ie.length();
        }

        length = length + self.rat_type.length();

        length = length + self.sender_f_teid_for_control_plane.length();

        if let Some(ref ie) = self.pgw_s5_s8_for_control_plane {
            length = length + ie.length();
        }

        length = length + self.apn.length();

        if let Some(ref ie) = self.selection_mode {
            length = length + ie.length();
        }

        for ie in &self.bearer_contexts_to_be_created {
            length = length + ie.length();
        }

        for ie in &self.bearer_contexts_to_be_removed {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.pdn_type {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.pdn_address_allocation {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.maximum_apn_restriction {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.apn_ambr {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.ue_time_zone {
            length = length + ie.length();
        }

        if let Some(ref ie) = self.charging_characteristics {
            length = length + ie.length();
        }

        length
    }
    fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = 0;

        if let Some(ref ie) = self.imsi {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.msisdn {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.mei {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.uli {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.serving_network {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        pos = pos + self.rat_type.generate(&mut buffer[pos..]);

        pos = pos + self.sender_f_teid_for_control_plane.generate(&mut buffer[pos..]);

        if let Some(ref ie) = self.pgw_s5_s8_for_control_plane {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        pos = pos + self.apn.generate(&mut buffer[pos..]);

        if let Some(ref ie) = self.selection_mode {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        for ie in &self.bearer_contexts_to_be_created {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        for ie in &self.bearer_contexts_to_be_removed {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.pdn_type {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.pdn_address_allocation {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.maximum_apn_restriction {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.apn_ambr {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.ue_time_zone {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        if let Some(ref ie) = self.charging_characteristics {
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
        apn,
        imsi,
        pdn_type,
        pdn_address_allocation,
        user_location_information,
        mei,
        serving_network,
        selection_mode,
        apn_restriction,
        ambr,
        ue_time_zone,
        charging_characteristics,
    };
    
    use crate::gtp_v2::packet::messages::information_elements::user_location_information::{PLMN, CGI, SAI, RAI, TAI, ECGI, LAI, MeNBID, EMeNBID};

    use ascii::{AsciiString, AsciiChar};

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let rat_type = rat_type::InformationElement::new(rat_type::RATType::EUTRAN, 0);

        let sender_f_teid_for_control_plane = f_teid::InformationElement::new(
            f_teid::InterfaceType::S11MmeGtpC,
            0x12345678, 
            Some(Ipv4Addr::new(10,0,0,1)), 
            None,
            0,
        );

        let bearer_qos = bearer_qos::InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_000,
            0,
            0,
            0
        );

        let eps_bearer_id = ebi::InformationElement::new(7, 0);

        let bearer_context_to_be_created = bearer_context::InformationElement::new(
            eps_bearer_id.unwrap(),
            bearer_qos.unwrap(),
            0
        );

        let apn = apn::InformationElement::new(
            AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap(),
            0
        );

        if bearer_context_to_be_created.is_ok() && rat_type.is_ok() && sender_f_teid_for_control_plane.is_ok() && apn.is_ok() {
            let mut m = Message::new(
                rat_type.unwrap(),
                sender_f_teid_for_control_plane.unwrap(),
                bearer_context_to_be_created.unwrap(),
                apn.unwrap()
            );

            m.imsi = Some(imsi::InformationElement::new("505013485090404", 0).unwrap());

            m.pdn_type = Some(pdn_type::InformationElement::new(pdn_type::PDNType::IPv6, 0).unwrap());

            m.pdn_address_allocation = Some(
                pdn_address_allocation::InformationElement::new(
                    pdn_type::PDNType::IPv4v6, 
                    Some(Ipv4Addr::new(10,0,0,1)), 
                    Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)), 
                    0
                ).unwrap()
            );

            m.msisdn = Some(msisdn::InformationElement::new("61123456789", 0).unwrap());

            m.uli = Some(
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

            m.mei = Some(mei::InformationElement::new([1,2,3,4,5,6,7,8,9,1,2,3,4,5,6], Some(1), 0).unwrap());

            m.serving_network = Some(serving_network::InformationElement::new(PLMN::new([5,0,5], [0,9,9]), 0).unwrap());

            m.selection_mode = Some(selection_mode::InformationElement::new(selection_mode::SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified, 0).unwrap());

            m.maximum_apn_restriction = Some(apn_restriction::InformationElement::new(apn_restriction::MaximumAPNRestrictionValue::Private2, 0).unwrap());

            m.apn_ambr = Some(ambr::InformationElement::new(0x12_34_56_78, 0x87_65_43_21, 0).unwrap());

            m.ue_time_zone = Some(ue_time_zone::InformationElement::new(0x40, ue_time_zone::DaylightSavingsTimeAdjustment::OneHourAdjustment, 0).unwrap()); // This is +10

            m.charging_characteristics = Some(charging_characteristics::InformationElement::new(0x1234, 0).unwrap());

            let expected = [
                InformationElementType::IMSI as u8,
                0, 8, // Length
                0, // Spare
                0x05, 0x05, 0x31, 0x84, 0x05, 0x09, 0x04, 0xF4,
                InformationElementType::MSISDN as u8,
                0, 6, // Length
                0, // Spare
                0x16, 0x21, 0x43, 0x65, 0x87, 0xF9,
                InformationElementType::MEI as u8,
                0, 8, // Length
                0, // Spare
                0x21, 0x43, 0x65, 0x87, 0x19, 0x32, 0x54, 0x16, //IMEISV
                InformationElementType::UserLocationInformation as u8,
                0, 51, // Length
                0, // Spare
                0b1111_1111, // Flags
                0x05, 0x95, 0x90, // PLMN
                0x12, 0x34, 0x43, 0x21, // LAC & CI
                0x05, 0x95, 0x90, // PLMN
                0x12, 0x34, 0x43, 0x21, // LAC & SAC
                0x05, 0x95, 0x90, // PLMN
                0x12, 0x34, 0x43, 0x21, // LAC & RAC
                0x05, 0x95, 0x90, // PLMN
                0x12, 0x34, // TAC
                0x05, 0x95, 0x90, // PLMN
                0x0F, 0xFF, 0xFF, 0xFF, // ECI
                0x05, 0x95, 0x90, // PLMN
                0x12, 0x34, // LAC
                0x05, 0x95, 0x90, // PLMN
                0x0F, 0xFF, 0xFF, // MeNBID
                0x05, 0x95, 0x90, // PLMN
                0x1F, 0xFF, 0xFF, // MeNBID
                InformationElementType::ServingNetwork as u8,
                0, 3, // Length
                0, // Spare
                0x05, 0x95, 0x90, // PLMN
                InformationElementType::RATType as u8,
                0, 1, // Length
                0, // Spare and Instance
                6, // RAT Type
                InformationElementType::FTEID as u8,
                0, 9, // Length
                0, // Spare and Instance
                (0b1 << 7) | (0b0 << 6) | (f_teid::InterfaceType::S11MmeGtpC as u8),
                0x12, 0x34, 0x56, 0x78,
                10, 0, 0, 1,
                InformationElementType::APN as u8,
                0, 31, // Length
                0, // Spare and Instance
                7, AsciiChar::a as u8, AsciiChar::w as u8, AsciiChar::e as u8, AsciiChar::s as u8, AsciiChar::o as u8, AsciiChar::m as u8,  AsciiChar::e as u8, 
                3, AsciiChar::a as u8, AsciiChar::p as u8, AsciiChar::n as u8,
                6, AsciiChar::m as u8, AsciiChar::n as u8, AsciiChar::c as u8, AsciiChar::_0 as u8,  AsciiChar::_9 as u8, AsciiChar::_9 as u8, 
                6, AsciiChar::m as u8, AsciiChar::c as u8, AsciiChar::c as u8, AsciiChar::_5 as u8,  AsciiChar::_0 as u8, AsciiChar::_5 as u8, 
                4, AsciiChar::g as u8, AsciiChar::p as u8,  AsciiChar::r as u8, AsciiChar::s as u8, 
                InformationElementType::SelectionMode as u8,
                0, 1, // Length
                0, // Spare
                selection_mode::SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified as u8, // Selection Mode
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
                InformationElementType::PDNType as u8,
                0, 1, // Length
                0, // Spare
                pdn_type::PDNType::IPv6 as u8, // PDN Type
                InformationElementType::PDNAddressAllocation as u8,
                0, 22, // Length
                0, // Spare
                pdn_type::PDNType::IPv4v6 as u8, // PDN Type
                128, // prefix
                0xFA, 0xDE, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xFE, 0xED, 0xDE, 0xAF, 0xBE, 0xAD, 0xFA, 0xCE,
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
                InformationElementType::UETimeZone as u8,
                0, 2, // Length
                0, // Spare
                0x04, // Timezone Offset
                0x1, // DST Adjustment
                InformationElementType::ChargingCharacteristics as u8,
                0, 2, // Length
                0, // Spare
                0x12, 0x34 // Charging Characteristics
            ];

            let pos = m.generate(&mut buffer);

            for i in 0..pos {
                if buffer[i] != expected[i] {
                    println!("{} (actual) != {} (expected) at byte {}", buffer[i], expected[i], i);
                    assert!(false);
                } 
            }
        }
        else { assert!(false) }
    }
    
    #[test]
    fn test_length() {
        let rat_type = rat_type::InformationElement::new(rat_type::RATType::EUTRAN, 0);

        let sender_f_teid_for_control_plane = f_teid::InformationElement::new(
            f_teid::InterfaceType::S11MmeGtpC,
            0x12345678, 
            Some(Ipv4Addr::new(10,0,0,1)), 
            None,
            0,
        );

        let bearer_qos = bearer_qos::InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_000,
            0,
            0,
            0
        );

        let eps_bearer_id = ebi::InformationElement::new(7, 0);

        let bearer_context_to_be_created = bearer_context::InformationElement::new(
            eps_bearer_id.unwrap(),
            bearer_qos.unwrap(),
            0
        );

        let apn = apn::InformationElement::new(
            AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap(),
            0
        );

        if bearer_context_to_be_created.is_ok() && rat_type.is_ok() && sender_f_teid_for_control_plane.is_ok() && apn.is_ok() {
            let mut m = Message::new(
                rat_type.unwrap(),
                sender_f_teid_for_control_plane.unwrap(),
                bearer_context_to_be_created.unwrap(),
                apn.unwrap()
            );

            m.imsi = Some(imsi::InformationElement::new("505013485080404", 0).unwrap());

            assert_eq!(m.length(), 100);

        }
        else { assert!(false) }
    }

    #[test]
    fn test_message_type() {
        let rat_type = rat_type::InformationElement::new(rat_type::RATType::EUTRAN, 0);

        let sender_f_teid_for_control_plane = f_teid::InformationElement::new(
            f_teid::InterfaceType::S11MmeGtpC,
            0x12345678, 
            Some(Ipv4Addr::new(10,0,0,1)), 
            None,
            0,
        );

        let bearer_qos = bearer_qos::InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_000,
            0,
            0,
            0
        );

        let eps_bearer_id = ebi::InformationElement::new(7, 0);

        let bearer_context_to_be_created = bearer_context::InformationElement::new(
            eps_bearer_id.unwrap(),
            bearer_qos.unwrap(),
            0
        );

        let apn = apn::InformationElement::new(
            AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap(),
            0
        );

        if bearer_context_to_be_created.is_ok() && rat_type.is_ok() && sender_f_teid_for_control_plane.is_ok() && apn.is_ok() {
            let m = Message::new(
                rat_type.unwrap(),
                sender_f_teid_for_control_plane.unwrap(),
                bearer_context_to_be_created.unwrap(),
                apn.unwrap()
            );

            assert_eq!(m.message_type() as u8, MessageType::CreateSessionRequest as u8)

        }
        else { assert!(false) }
    }

    #[test]
    fn test_message_parse() {
        let message_bytes = [
            InformationElementType::IMSI as u8,
            0, 8, // Length
            0, // Spare and Instance
            0x05, 0x05, 0x31, 0x84, 0x05, 0x09, 0x04, 0xF4,
            InformationElementType::MSISDN as u8,
            0, 6, // Length
            0, // Spare
            0x16, 0x21, 0x43, 0x65, 0x87, 0xF9,
            InformationElementType::MEI as u8,
            0, 8, // Length
            0, // Spare
            0x21, 0x43, 0x65, 0x87, 0x19, 0x32, 0x54, 0x16, //IMEISV
            InformationElementType::RATType as u8,
            0, 1, // Length
            0, // Spare and Instance
            6, // RAT Type
            InformationElementType::FTEID as u8,
            0, 9, // Length
            0, // Spare and Instance
            (0b1 << 7) | (0b0 << 6) | (f_teid::InterfaceType::S11MmeGtpC as u8),
            0x12, 0x34, 0x56, 0x78,
            10, 0, 0, 1,
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
            0b00100101, // Flags
            7, // QCI
            0x00, 0x00, 0x98, 0x96, 0x80,
            0x00, 0x00, 0x98, 0x96, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
            InformationElementType::APN as u8,
            0, 31, // Length
            0, // Spare and Instance
            7, AsciiChar::a as u8, AsciiChar::w as u8,  AsciiChar::e as u8, AsciiChar::s as u8, AsciiChar::o as u8, AsciiChar::m as u8,  AsciiChar::e as u8, 
            3, AsciiChar::a as u8, AsciiChar::p as u8, AsciiChar::n as u8,
            6, AsciiChar::m as u8, AsciiChar::n as u8, AsciiChar::c as u8, AsciiChar::_0 as u8,  AsciiChar::_9 as u8, AsciiChar::_9 as u8, 
            6, AsciiChar::m as u8, AsciiChar::c as u8, AsciiChar::c as u8, AsciiChar::_5 as u8,  AsciiChar::_0 as u8, AsciiChar::_5 as u8, 
            4, AsciiChar::g as u8, AsciiChar::p as u8,  AsciiChar::r as u8, AsciiChar::s as u8, 
            InformationElementType::PDNType as u8,
            0, 1, // Length
            0, // Spare
            pdn_type::PDNType::IPv6 as u8, // PDN Type
            InformationElementType::PDNAddressAllocation as u8,
            0, 18, // Length
            0, // Spare
            pdn_type::PDNType::IPv6 as u8, // PDN Type
            128, // Prefix
            0xFA, 0xDE, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xFE, 0xED, 0xDE, 0xAF, 0xBE, 0xAD, 0xFA, 0xCE, // IPv6 Address
            InformationElementType::ServingNetwork as u8,
            0, 3, // Length
            0, // Spare
            0x05, 0x95, 0x90, // PLMN
            InformationElementType::SelectionMode as u8,
            0, 1, // Length
            0, // Spare
            selection_mode::SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified as u8, // Selection Mode
            InformationElementType::APNRestriction as u8,
            0, 1, // Length
            0, // Spare
            4, // APN Restriction
            InformationElementType::AMBR as u8,
            0, 8, // Length
            0, // Spare
            0x12, 0x34, 0x56, 0x78, // AMBR for uplink
            0x87, 0x65, 0x43, 0x21, // AMBR for uplink
            InformationElementType::UETimeZone as u8,
            0, 2, // Length
            0, // Spare
            0x04, // Timezone Offset
            0x1, // DST Adjustment
            InformationElementType::ChargingCharacteristics as u8,
            0, 2, // Length
            0, // Spare
            0x12, 0x34 // Charging Characteristics
        ];

        if let Some((m, _pos)) = Message::parse(&message_bytes){

            if let Some(i) = m.imsi {
                assert_eq!(i.imsi, [5, 0, 5, 0, 1, 3, 4, 8, 5, 0, 9, 0, 4, 0, 4]);
            }
            else { assert!(false); }

            if let Some(i) = m.msisdn {
                assert_eq!(i.msisdn, [6, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
            }
            else { assert!(false); }

            assert_eq!(m.rat_type.rat_type as u8, rat_type::RATType::EUTRAN as u8);
            
            if let Some(a) = m.sender_f_teid_for_control_plane.ipv4_address {
                assert_eq!(a, Ipv4Addr::new(10,0,0,1));
            }
            else {
                assert!(false);
            }
            assert_eq!(m.sender_f_teid_for_control_plane.teid, 0x12345678);
            assert_eq!(m.sender_f_teid_for_control_plane.interface_type as u8, f_teid::InterfaceType::S11MmeGtpC as u8);
            
            assert_eq!(m.bearer_contexts_to_be_created[0].eps_bearer_id.eps_bearer_id, 7);
            assert_eq!(m.bearer_contexts_to_be_created[0].bearer_level_qos.pvi, true);
            assert_eq!(m.bearer_contexts_to_be_created[0].bearer_level_qos.pl(), 9);
            assert_eq!(m.bearer_contexts_to_be_created[0].bearer_level_qos.pci, false);
            assert_eq!(m.bearer_contexts_to_be_created[0].bearer_level_qos.qci, 7);
            assert_eq!(m.bearer_contexts_to_be_created[0].bearer_level_qos.guaranteed_ul_bitrate(), 0);
            assert_eq!(m.bearer_contexts_to_be_created[0].bearer_level_qos.guaranteed_dl_bitrate(), 0);
            assert_eq!(m.bearer_contexts_to_be_created[0].bearer_level_qos.max_ul_bitrate(), 10_000_000);
            assert_eq!(m.bearer_contexts_to_be_created[0].bearer_level_qos.max_dl_bitrate(), 10_000_000);

            assert_eq!(m.apn.apn, AsciiString::from_ascii("awesome.apn.mnc099.mcc505.gprs").unwrap());

            if let Some(pdn_type) = m.pdn_type {
                assert_eq!(pdn_type.pdn_type as u8, pdn_type::PDNType::IPv6 as u8);
            }
            else { assert!(false); }

            if let Some(pdn_address_allocation) = m.pdn_address_allocation {
                assert_eq!(pdn_address_allocation.pdn_type as u8, pdn_type::PDNType::IPv6 as u8);
                assert_eq!(pdn_address_allocation.ipv6_address_and_prefix, Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)));
            }
            else { assert!(false); }

            if let Some(mei) = m.mei {
                assert_eq!(mei.imei, [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6]);
                assert!(mei.sv.is_some());
                assert_eq!(mei.sv.unwrap(), 1);
            }
            else { assert!(false); }

            if let Some(ie) = m.serving_network {
                assert_eq!(ie.plmn, PLMN::new([5,0,5],[0,9,9]));
            }
            else { assert!(false); }

            if let Some(ie) = m.selection_mode {
                assert_eq!(ie.selection_mode, selection_mode::SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified);
            }
            else { assert!(false); }

            if let Some(ie) = m.maximum_apn_restriction {
                assert_eq!(ie.maximum_apn_restriction, apn_restriction::MaximumAPNRestrictionValue::Private2);
            }
            else { assert!(false); }

            if let Some(ie) = m.apn_ambr {
                assert_eq!(ie.uplink, 0x12_34_56_78);
                assert_eq!(ie.uplink, 0x87_65_43_21);
            }
            else { assert!(false); }

            if let Some(ie) = m.ue_time_zone {
                assert_eq!(ie.timezone_offset, 0x40);
                assert_eq!(ie.dst_adjustment, ue_time_zone::DaylightSavingsTimeAdjustment::OneHourAdjustment);
            }
            else { assert!(false); }

            if let Some(ie) = m.charging_characteristics {
                assert_eq!(ie.charging_characteristics, 0x1234);
            }
            else { assert!(false); }
        }
    }
}
