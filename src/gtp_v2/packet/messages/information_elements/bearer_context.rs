use byteorder::{ByteOrder, NetworkEndian};

use std::convert::TryFrom;

use std::net::{Ipv4Addr, Ipv6Addr};

use super::{InformationElementTraits, InformationElementType, InformationElement as IEEnum, LENGTH, bearer_qos, f_teid, ebi};

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (93)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       | IEs                                                           |
                |---------------------------------------------------------------|

        ----------------------------|---------------------------|---------------|------------------------                             
        Information Element         |   Presence requirement    | Instance      |  Reference 
        ----------------------------|---------------------------|---------------|------------------------
        EPS Bearer ID               |   Mandatory               | 0             | 8.8
        TFT                         |   Optional                | 0             |  
        S1-U eNodeB F-TEID          |   Conditional             | 0             | 8.22
        S4-U SGSN F-TEID            |   Conditional             | 1             | 8.22
        S5/S8-U SGW F-TEID          |   Conditional             | 2             | 8.22
        S5/S8-U PGW F-TEID          |   Conditional             | 3             | 8.22
        S12 RNC F-TEID              |   Conditional             | 4             | 8.22
        S2b-U ePDG F-TEID           |   Conditional             | 5             | 8.22
        S2a-U TWAN F-TEID           |   Conditional             | 6             | 8.22
        Bearer Level QoS            |   Mandatory               | 0             | 8.15
        S11-U MME F-TEID            |   Conditional             | 7             | 8.22
    */

    instance: u8,
    pub eps_bearer_id: ebi::InformationElement,
    pub s1_u_enodeb_f_teid: Option<f_teid::InformationElement>,
    pub s4_u_sgsn_f_teid: Option<f_teid::InformationElement>,
    pub s5_s8_u_sgw_f_teid: Option<f_teid::InformationElement>,
    pub s5_s8_u_pgw_f_teid: Option<f_teid::InformationElement>,
    pub s12_rnc_f_teid: Option<f_teid::InformationElement>,
    pub s2b_u_epdg_f_teid: Option<f_teid::InformationElement>,
    pub s2a_u_twan_f_teid: Option<f_teid::InformationElement>,
    pub s11_u_mme_f_teid: Option<f_teid::InformationElement>,
    pub bearer_level_qos: bearer_qos::InformationElement
}

#[derive(Copy, Clone, Debug)]
enum FTeidInstance {
    S1UENodeB = 0,
    S4USgsn = 1,
    S5S8USgw = 2,
    S5S8UPgw = 3,
    S12Rnc = 4,
    S2bUEPdg = 5,
    S2aUTwan = 6,
    S11UMme = 7,
}

impl TryFrom<u8> for FTeidInstance
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(FTeidInstance::S1UENodeB),
            1 => Ok(FTeidInstance::S4USgsn),
            2 => Ok(FTeidInstance::S5S8USgw),
            3 => Ok(FTeidInstance::S5S8UPgw),
            4 => Ok(FTeidInstance::S12Rnc),
            5 => Ok(FTeidInstance::S2bUEPdg),
            6 => Ok(FTeidInstance::S2aUTwan),
            7 => Ok(FTeidInstance::S11UMme),
            _ => Err(format!("Unsupported F-TEID Instance ({})", value))
        }
    }
}

impl InformationElement {
    pub fn new(
        eps_bearer_id: ebi::InformationElement,
        bearer_level_qos: bearer_qos::InformationElement,
        instance: u8,
    ) -> Result<Self,String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    eps_bearer_id,
                    bearer_level_qos,
                    instance,
                    s1_u_enodeb_f_teid: None,
                    s4_u_sgsn_f_teid: None,
                    s5_s8_u_sgw_f_teid: None,
                    s5_s8_u_pgw_f_teid: None,
                    s12_rnc_f_teid: None,
                    s2b_u_epdg_f_teid: None,
                    s2a_u_twan_f_teid: None,
                    s11_u_mme_f_teid: None,
                }
            )
        }
    }

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        // Keep trying to parse IEs until the end of the buffer
        let mut eps_bearer_id: Option<ebi::InformationElement> = None;

        let mut bearer_level_qos: Option<bearer_qos::InformationElement> = None;

        let mut s1_u_enodeb_f_teid: Option<f_teid::InformationElement> = None;
        let mut s4_u_sgsn_f_teid: Option<f_teid::InformationElement> = None;
        let mut s5_s8_u_sgw_f_teid: Option<f_teid::InformationElement> = None;
        let mut s5_s8_u_pgw_f_teid: Option<f_teid::InformationElement> = None;
        let mut s12_rnc_f_teid: Option<f_teid::InformationElement> = None;
        let mut s2b_u_epdg_f_teid: Option<f_teid::InformationElement> = None;
        let mut s2a_u_twan_f_teid: Option<f_teid::InformationElement> = None;
        let mut s11_u_mme_f_teid: Option<f_teid::InformationElement> = None;

        let mut pos = 0;

        // Read the type
        let _ie_type = buffer[pos];
        pos = pos + 1;

        // Read the length
        let length = NetworkEndian::read_u16(&buffer[LENGTH]);
        pos = pos + 2;

        //Spare and instance
        let instance = buffer[pos] & 0xF;
        pos = pos + 1;

        while pos < buffer.len()
        {
            if let Some((ie, ie_pos)) = IEEnum::parse(&buffer[pos..]){
                match ie {
                    IEEnum::EBI(ie) => eps_bearer_id = Some(ie),
                    IEEnum::BearerQoS(ie) => bearer_level_qos = Some(ie),
                    IEEnum::FTEID(ie) => {
                        if let Ok(instance) = FTeidInstance::try_from(ie.instance()) {
                            match instance
                            {
                                FTeidInstance::S1UENodeB => s1_u_enodeb_f_teid = Some(ie),
                                FTeidInstance::S4USgsn => s4_u_sgsn_f_teid = Some(ie),
                                FTeidInstance::S5S8USgw => s5_s8_u_sgw_f_teid = Some(ie),
                                FTeidInstance::S5S8UPgw => s5_s8_u_pgw_f_teid = Some(ie),
                                FTeidInstance::S12Rnc => s12_rnc_f_teid = Some(ie),
                                FTeidInstance::S2bUEPdg => s2b_u_epdg_f_teid = Some(ie),
                                FTeidInstance::S2aUTwan => s2a_u_twan_f_teid = Some(ie),
                                FTeidInstance::S11UMme => s11_u_mme_f_teid = Some(ie),
                            }
                        }
                        else { /* Not an instance of F-TEID that we expect. Just ignore it */ }
                    },
                    _ =>  { /* Its an IE that we didn't expect. Just ignore it */ }
                }
                pos = pos + ie_pos;
            }
            else {
                // IE parsing failed
                pos = pos + IEEnum::skip_parsing(&buffer[pos..]);
            }
        }

        if eps_bearer_id.is_some() && bearer_level_qos.is_some() {
            Some(
                (
                    InformationElement {
                        eps_bearer_id: eps_bearer_id.unwrap(),
                        bearer_level_qos: bearer_level_qos.unwrap(),
                        instance,
                        s1_u_enodeb_f_teid: s1_u_enodeb_f_teid,
                        s4_u_sgsn_f_teid: s4_u_sgsn_f_teid,
                        s5_s8_u_sgw_f_teid: s5_s8_u_sgw_f_teid,
                        s5_s8_u_pgw_f_teid: s5_s8_u_pgw_f_teid,
                        s12_rnc_f_teid: s12_rnc_f_teid,
                        s2b_u_epdg_f_teid: s2b_u_epdg_f_teid,
                        s2a_u_twan_f_teid: s2a_u_twan_f_teid,
                        s11_u_mme_f_teid: s11_u_mme_f_teid,
                    },
                    (length + 4) as usize
                )
            )
        }
        else {
            None
        }
    }

    pub fn set_s1_u_enodeb_f_teid(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.s1_u_enodeb_f_teid = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S1UENodeBGtpU,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::S1UENodeB as u8
        ).unwrap());
    }

    pub fn unset_s1_u_enodeb_f_teid(&mut self) {
        self.s1_u_enodeb_f_teid = None;
    }

    pub fn set_s4_u_sgsn_f_teid(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.s4_u_sgsn_f_teid = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S4SgsnGtpU,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::S4USgsn as u8
        ).unwrap());
    }
    pub fn unset_s4_u_sgsn_f_teid(&mut self) {
        self.s4_u_sgsn_f_teid = None;
    }

    pub fn set_s5_s8_u_sgw_f_teid(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.s5_s8_u_sgw_f_teid = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S5S8SgwGtpU,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::S5S8USgw as u8
        ).unwrap());
    }

    pub fn set_s5_s8_u_pgw_f_teid(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.s5_s8_u_pgw_f_teid = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S5S8PgwGtpU,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::S5S8UPgw as u8
        ).unwrap());
    }
    pub fn unset_s5_s8_u_pgw_f_teid(&mut self) {
        self.s5_s8_u_pgw_f_teid = None;
    }

    pub fn unset_s5_s8_u_sgw_f_teid(&mut self) {
        self.s5_s8_u_sgw_f_teid = None;
    }

    pub fn set_s12_rnc_f_teid(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.s12_rnc_f_teid = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S12RncGtpU,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::S12Rnc as u8
        ).unwrap());
    }
    pub fn unset_s12_rnc_f_teid(&mut self) {
        self.s12_rnc_f_teid = None;
    }

    pub fn set_s2b_u_epdg_f_teid(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.s2b_u_epdg_f_teid = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S2bUEPdgGtpU,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::S2bUEPdg as u8
        ).unwrap());
    }
    pub fn unset_s2b_u_epdg_f_teid(&mut self) {
        self.s2b_u_epdg_f_teid = None;
    }

    pub fn set_s2a_u_twan_f_teid(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.s2a_u_twan_f_teid = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S2aTwanGtpU,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::S2aUTwan as u8
        ).unwrap());
    }
    pub fn unset_s2a_u_twan_f_teid(&mut self) -> &Self {
        self.s2a_u_twan_f_teid = None;
        
        self
    }

    pub fn set_s11_u_mme_f_teid(&mut self, teid: u32, ipv4_address: Option<Ipv4Addr>, ipv6_address: Option<Ipv6Addr>) {
        self.s11_u_mme_f_teid = Some(f_teid::InformationElement::new(
            f_teid::InterfaceType::S11MmeGtpU,
            teid,
            ipv4_address,
            ipv6_address,
            FTeidInstance::S11UMme as u8
        ).unwrap());
    }
    pub fn unset_s11_u_mme_f_teid(&mut self) {
        self.s11_u_mme_f_teid = None;
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::BearerContext
    }

    fn instance(&self) -> u8 {
        self.instance
    }

    fn set_instance(&mut self, instance: u8) -> Result<u8, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            self.instance = instance;
            Ok(self.instance)
        }
    }

    fn length(&self) -> u16 {
        /* This is the actual length of the Information element INCLUDING the first 4 octets
        To calculate the length field of the IE you need to subtract 4 from what is returned */

        let mut length = 4;

        length = length + self.eps_bearer_id.length();

        if let Some(ref ie) = self.s1_u_enodeb_f_teid {
            length = length + ie.length();
        }
        if let Some(ref ie) = self.s4_u_sgsn_f_teid {
            length = length + ie.length();
        }
        if let Some(ref ie) = self.s5_s8_u_sgw_f_teid {
            length = length + ie.length();
        }
        if let Some(ref ie) = self.s5_s8_u_pgw_f_teid {
            length = length + ie.length();
        }
        if let Some(ref ie) = self.s12_rnc_f_teid {
            length = length + ie.length();
        }
        if let Some(ref ie) = self.s2b_u_epdg_f_teid {
            length = length + ie.length();
        }
        if let Some(ref ie) = self.s2a_u_twan_f_teid {
            length = length + ie.length();
        }

        length = length + self.bearer_level_qos.length();

        if let Some(ref ie) = self.s11_u_mme_f_teid {
            length = length + ie.length();
        }

        length
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = 0;
        
        // Write the type
        buffer[pos] = self.information_element_type() as u8;
        pos = pos + 1;

        // Write the length
        // We subtract 4 octets as the type and length fields aren't included.
        NetworkEndian::write_u16(&mut buffer[LENGTH], self.length()-4);
        pos = pos + 2;

        //Spare and instance
        buffer[pos] = self.instance & 0xF;
        pos = pos + 1;

        pos = pos + self.eps_bearer_id.generate(&mut buffer[pos..]);
        
        if let Some(ref ie) = self.s1_u_enodeb_f_teid {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }
        if let Some(ref ie) = self.s4_u_sgsn_f_teid {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }
        if let Some(ref ie) = self.s5_s8_u_sgw_f_teid {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }
        if let Some(ref ie) = self.s5_s8_u_pgw_f_teid {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }
        if let Some(ref ie) = self.s12_rnc_f_teid {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }
        if let Some(ref ie) = self.s2b_u_epdg_f_teid {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }
        if let Some(ref ie) = self.s2a_u_twan_f_teid {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        pos = pos + self.bearer_level_qos.generate(&mut buffer[pos..]);

        if let Some(ref ie) = self.s11_u_mme_f_teid {
            pos = pos + ie.generate(&mut buffer[pos..]);
        }

        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    use crate::gtp_v2::packet::messages::information_elements::InformationElementType;

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

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

        if let Ok(bearer_qos) = bearer_qos {
            if let Ok(eps_bearer_id) = eps_bearer_id {
                if let Ok(ie) = InformationElement::new(
                    eps_bearer_id,
                    bearer_qos,
                    0
                ) {
                    let pos = ie.generate(&mut buffer);

                    let expected = [
                        InformationElementType::BearerContext as u8,
                        0, 31, // Length
                        0, // Spare
                        InformationElementType::EBI as u8,
                        0, 1, // Length
                        0, // Spare
                        7, // EPS Bearer ID
                        InformationElementType::BearerQoS as u8,
                        0, 22, // Length
                        0, // Spare
                        0b01100100, // Flags
                        7, // QCI
                        0x00, 0x00, 0x98, 0x96, 0x80,
                        0x00, 0x00, 0x98, 0x96, 0x80,
                        0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00,
                    ];
    
                    for i in 0..pos {
                        if buffer[i] != expected[i] {
                            println!("{} (actual) != {} (expected) at byte {}", buffer[i], expected[i], i);
                            assert!(false);
                        } 
                    }
                }
                else { assert!(false); }
            }
            else { assert!(false); }
        }
        else { assert!(false); }
    }

    #[test]
    fn test_length() {
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

        if let Ok(bearer_qos) = bearer_qos {
            if let Ok(eps_bearer_id) = eps_bearer_id {
                if let Ok(ie) = InformationElement::new(
                    eps_bearer_id,
                    bearer_qos,
                    0
                ) {
                    assert_eq!(ie.length(), 35);
                }
                else { assert!(false); }
            }
            else { assert!(false); }
        }
        else { assert!(false); }
    }

    #[test]
    fn test_message_type() {
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

        if let Ok(bearer_qos) = bearer_qos {
            if let Ok(eps_bearer_id) = eps_bearer_id {
                if let Ok(ie) = InformationElement::new(
                    eps_bearer_id,
                    bearer_qos,
                    0
                ) {
                    assert_eq!(ie.information_element_type() as u8, InformationElementType::BearerContext as u8);
                }
                else { assert!(false); }
            }
            else { assert!(false); }
        }
        else { assert!(false); }
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [
            InformationElementType::BearerContext as u8,
            0, 31, // Length
            0, // Spare
            InformationElementType::EBI as u8,
            0, 1, // Length
            0, // Spare
            7, // EPS Bearer ID
            InformationElementType::BearerQoS as u8,
            0, 22, // Length
            0, // Spare
            0b00100101, // Flags
            7, // QCI
            0x00, 0x00, 0x98, 0x96, 0x80,
            0x00, 0x00, 0x98, 0x96, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.eps_bearer_id.eps_bearer_id, 7);
            assert_eq!(ie.bearer_level_qos.pvi, true);
            assert_eq!(ie.bearer_level_qos.pl(), 9);
            assert_eq!(ie.bearer_level_qos.pci, false);
            assert_eq!(ie.bearer_level_qos.qci, 7);
            assert_eq!(ie.bearer_level_qos.guaranteed_ul_bitrate(), 0);
            assert_eq!(ie.bearer_level_qos.guaranteed_dl_bitrate(), 0);
            assert_eq!(ie.bearer_level_qos.max_ul_bitrate(), 10_000_000);
            assert_eq!(ie.bearer_level_qos.max_dl_bitrate(), 10_000_000);
        }
        else {
            assert!(false);
        }
    }
}