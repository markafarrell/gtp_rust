use super::{
    MessageTraits, 
    MessageType,
};

use super::information_elements::{InformationElementTraits, InformationElement};

pub struct Message {
    /*
        --------------------------------------------|---------------------------|-------------------------                             
        Information Element                         |   Presence requirement    |   Reference 
        --------------------------------------------|---------------------------|-------------------------
        IMSI                                        |    Conditional            |   7.7.2
        Routeing Area Identity (RAI)                |    Optional               |   7.7.3
        Recovery                                    |    Optional               |   7.7.11
        Selection mode                              |    Conditional            |   7.7.12
        Tunnel Endpoint Identifier Data I           |    Mandatory              |   7.7.13
        Tunnel Endpoint Identifier Control Plane    |    Conditional            |   7.7.14
        NSAPI                                       |    Mandatory              |   7.7.17
        Linked NSAPI                                |    Conditional            |   7.7.17
        Charging Characteristics                    |    Conditional            |   7.7.23
        Trace Reference                             |    Optional               |   7.7.24
        Trace Type                                  |    Optional               |   7.7.25
        End User Address                            |    Conditional            |   7.7.27
        Access Point Name                           |    Conditional            |   7.7.30
        Protocol Configuration Options              |    Optional               |   7.7.31
        SGSN Address for signalling                 |    Mandatory              |   GSN Address 7.7.32
        SGSN Address for user traffic               |    Mandatory              |   GSN Address 7.7.32
        MSISDN                                      |    Conditional            |   7.7.33
        Quality of Service Profile                  |    Mandatory              |   7.7.34
        TFT                                         |    Conditional            |   7.7.36
        Trigger Id                                  |    Optional               |   7.7.41
        OMC Identity                                |    Optional               |   7.7.42
        Common Flags                                |    Optional               |   7.7.48
        APN Restriction                             |    Optional               |   7.7.49
        RAT Type                                    |    Optional               |   7.7.50
        User Location Information                   |    Optional               |   7.7.51
        MS Time Zone                                |    Optional               |   7.7.52
        IMEI(SV)                                    |    Conditional            |   7.7.53
        CAMEL Charging Information Container        |    Optional               |   7.7.54
        Additional Trace Info                       |    Optional               |   7.7.62
        Correlation-ID                              |    Optional               |   7.7.82
        Evolved Allocation/Retention Priority I     |    Optional               |   7.7.91
        Extended Common Flags                       |    Optional               |   7.7.93
        User CSG Information                        |    Optional               |   7.7.94
        APN-AMBR                                    |    Optional               |   7.7.98
        Signalling Priority Indication              |    Optional               |   7.7.103
        CN Operator Selection Entity                |    Optional               |   7.7.116
        Mapped UE Usage Type                        |    Optional               |   7.7.123
        UP Function Selection Indication Flags      |    Optional               |   7.7.124
        Private Extension                           |    Optional               |   7.7.46
        --------------------------------------------|---------------------------|-------------------------
    */
    pub information_elements: Vec<InformationElement>
}

impl Message {
    pub fn new() -> Self {
        
        Message {
            information_elements: Vec::new()
        }
    }
    pub fn parse(_buffer: &[u8]) -> Option<(Self, usize)> {
        None
    }
}

impl MessageTraits for Message {
    fn push_ie(&mut self, ie: InformationElement)
    {
        // TODO: Check here that the ie we are adding is allowed for this message
        self.information_elements.push(ie);
    }

    fn pop_ie(&mut self) -> Option<InformationElement>
    {
        self.information_elements.pop()
    }

    fn message_type(&self) -> MessageType {
        MessageType::CreatePDPContextRequest
    }

    fn length(&self) -> u16 {
        let mut length = 0;

        for ie in self.information_elements.iter() {
            length = length + ie.length();
        }

        length
    }
    fn generate(&self, buffer: &mut[u8]) -> usize {
        // NOTE: The list should be sorted by IE Type. We assume here they have been added in the correct order
        let mut pos = 0;

        for ie in self.information_elements.iter() {
            let ie_size = ie.generate(&mut buffer[pos..]);

            pos = pos + ie_size;
        }

        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use crate::gtp_v1::packet::messages::{
        MessageTraits
    };

    use crate::gtp_v1::packet::messages::information_elements;
    use crate::gtp_v1::packet::messages::information_elements::InformationElementType;

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let mut m = Message::new();

        m.information_elements.push(
            InformationElement::TeidDataI(information_elements::teid_data_i::InformationElement::new(0x12345678))
        );

        let nsapi = information_elements::nsapi::InformationElement::new(0xF);

        if let Ok(nsapi) = nsapi {
            m.information_elements.push(
                InformationElement::Nsapi(nsapi)
            );
        }
        
        m.information_elements.push(
            InformationElement::GsnAddress(
                information_elements::gsn_address::InformationElement::new(
                    IpAddr::V4(
                        Ipv4Addr::new(192,168,0,1)
                    )
                )
            )
        );
        
        m.information_elements.push(
            InformationElement::GsnAddress(
                information_elements::gsn_address::InformationElement::new(
                    IpAddr::V6(
                        Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE)
                    )
                )
            )
        );

        m.information_elements.push(
            InformationElement::QoSProfile(
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

        let pos = m.generate(&mut buffer);

        let expected = [
            InformationElementType::TeidDataI as u8, 0x12, 0x34, 0x56, 0x78,
            InformationElementType::Nsapi as u8, 0xF,
            InformationElementType::GsnAddress as u8, 0, 4, 192, 168, 0, 1,
            InformationElementType::GsnAddress as u8, 0, 16, 0xFA, 0xDE, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xFE, 0xED, 0xDE, 0xAF, 0xBE, 0xAD, 0xFA, 0xCE,
            InformationElementType::QoSProfile as u8, 0, 4, 8, 0b0010_0101, 0b0001_0010, 0x1F,
        ];

        for i in 0..pos {
            if buffer[i] != expected[i] {
                println!("{} (actual) != {} (expected) at byte {}", buffer[i], expected[i], i);
                assert!(false);
            } 
        }
    }
    
    #[test]
    fn test_length() {
        let mut m = Message::new();

        m.information_elements.push(
            InformationElement::TeidDataI(
                information_elements::teid_data_i::InformationElement::new(0x12345678)
            )
        );

        let nsapi = information_elements::nsapi::InformationElement::new(0xF);

        if let Ok(nsapi) = nsapi {
            m.information_elements.push(
                InformationElement::Nsapi(nsapi)
            );
        }
        
        m.information_elements.push(
            InformationElement::GsnAddress(
                information_elements::gsn_address::InformationElement::new(
                    IpAddr::V4(
                        Ipv4Addr::new(192,168,0,1)
                    )
                )
            )
        );
        
        m.information_elements.push(
            InformationElement::GsnAddress(
                information_elements::gsn_address::InformationElement::new(
                    IpAddr::V6(
                        Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE)
                    )
                )
            )
        );

        m.information_elements.push(
            InformationElement::QoSProfile(
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

        assert_eq!(m.length(), 40);
    }

    #[test]
    fn test_message_type() {
        let m = Message::new();
        assert_eq!(m.message_type() as u8, MessageType::CreatePDPContextRequest as u8)
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}
