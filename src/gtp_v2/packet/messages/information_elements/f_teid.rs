extern crate rand;

use byteorder::{ByteOrder, NetworkEndian};

use std::convert::TryFrom;

use std::net::{Ipv4Addr, Ipv6Addr};

use super::{InformationElementTraits, InformationElementType, LENGTH};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InterfaceType {
    S1UENodeBGtpU = 0,
    S1USgwGtpU = 1,
    S12RncGtpU = 2,
    S12SgwGtpU = 3,
    S5S8SgwGtpU = 4,
    S5S8PgwGtpU = 5,
    S5S8SgwGtpC = 6,
    S5S8PgwGtpC = 7,
    S5S8SgwPmipV6 = 8,
    S5S8PgwPmipV6 = 9,
    S11MmeGtpC = 10,
    S11S4SgwGtpC = 11,
    S10N26MmeGtpC = 12,
    S3MmeGtpC = 13,
    S3SgsnGtpC = 14,
    S4SgsnGtpU = 15,
    S4SgwGtpU = 16,
    S4SgsnGtpC = 17,
    S16SgsnGtpC = 18,
    ENodeBGtpUForDlDataForwarding = 19,
    ENodeBGtpUForUlDataForwarding = 20,
    RncGtpUForDataForwarding = 21,
    SgsnGtpUForDataForwarding = 22,
    SgwUpfGtpUForDlDataForwarding = 23,
    SmMbmsGwGtpC = 24,
    SnMbmsGwGtpC = 25,
    SmMmeGtpC = 26,
    SnSgsnGtpC = 27,
    SgwGtpUForUlDataForwarding = 28,
    SnSgsnGtpU = 29,
    S2bEPdgGtpC = 30,
    S2bUEPdgGtpU = 31,
    S2bPgwGtpC = 32,
    S2bUPgwGtpU = 33,
    S2aTwanGtpU = 34,
    S2aTwanGtpC = 35,
    S2aPgwGtpC = 36,
    S2aPgwGtpU = 37,
    S11MmeGtpU = 38,
    S11SgwGtpU = 39,
    N26AmfGtpC = 40,
}

impl TryFrom<u8> for InterfaceType
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(InterfaceType::S1USgwGtpU),
            1 => Ok(InterfaceType::S1USgwGtpU),
            2 => Ok(InterfaceType::S12RncGtpU),
            3 => Ok(InterfaceType::S12SgwGtpU),
            4 => Ok(InterfaceType::S5S8SgwGtpU),
            5 => Ok(InterfaceType::S5S8PgwGtpU),
            6 => Ok(InterfaceType::S5S8SgwGtpC),
            7 => Ok(InterfaceType::S5S8PgwGtpC),
            8 => Ok(InterfaceType::S5S8SgwPmipV6),
            9 => Ok(InterfaceType::S5S8PgwPmipV6),
            10 => Ok(InterfaceType::S11MmeGtpC),
            11 => Ok(InterfaceType::S11S4SgwGtpC),
            12 => Ok(InterfaceType::S10N26MmeGtpC),
            13 => Ok(InterfaceType::S3MmeGtpC),
            14 => Ok(InterfaceType::S3SgsnGtpC),
            15 => Ok(InterfaceType::S4SgsnGtpU),
            16 => Ok(InterfaceType::S4SgwGtpU),
            17 => Ok(InterfaceType::S4SgsnGtpC),
            18 => Ok(InterfaceType::S16SgsnGtpC),
            19 => Ok(InterfaceType::ENodeBGtpUForDlDataForwarding),
            20 => Ok(InterfaceType::ENodeBGtpUForUlDataForwarding),
            21 => Ok(InterfaceType::RncGtpUForDataForwarding),
            22 => Ok(InterfaceType::SgsnGtpUForDataForwarding),
            23 => Ok(InterfaceType::SgwUpfGtpUForDlDataForwarding),
            24 => Ok(InterfaceType::SmMbmsGwGtpC),
            25 => Ok(InterfaceType::SnMbmsGwGtpC),
            26 => Ok(InterfaceType::SmMmeGtpC),
            27 => Ok(InterfaceType::SnSgsnGtpC),
            28 => Ok(InterfaceType::SgwGtpUForUlDataForwarding),
            29 => Ok(InterfaceType::SnSgsnGtpU),
            30 => Ok(InterfaceType::S2bEPdgGtpC),
            31 => Ok(InterfaceType::S2bUEPdgGtpU),
            32 => Ok(InterfaceType::S2bPgwGtpC),
            33 => Ok(InterfaceType::S2bUPgwGtpU),
            34 => Ok(InterfaceType::S2aTwanGtpU),
            35 => Ok(InterfaceType::S2aTwanGtpC),
            36 => Ok(InterfaceType::S2aPgwGtpC),
            37 => Ok(InterfaceType::S2aPgwGtpU),
            38 => Ok(InterfaceType::S11MmeGtpU),
            39 => Ok(InterfaceType::S11SgwGtpU),
            40 => Ok(InterfaceType::N26AmfGtpC),
            _ => Err(format!("Unsupported Interface type ({})", value))
        }
    }
}

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (87)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       | V4    | V6    | Interface Type                                |
        6       | TEID/GRE Key (Octet 1)                                        |
        7       | TEID/GRE Key (Octet 2)                                        |
        8       | TEID/GRE Key (Octet 3)                                        |
        9       | TEID/GRE Key (Octet 4)                                        |
        10      | IPv4 Address (Octet 1) (Present if V4 FLAG is set)            |
        11      | IPv4 Address (Octet 2) (Present if V4 FLAG is set)            |
        12      | IPv4 Address (Octet 3) (Present if V4 FLAG is set)            |
        13      | IPv4 Address (Octet 4) (Present if V4 FLAG is set)            |
        14      | IPv6 Address (Octet 1) (Present if V6 FLAG is set)            |
        15      | IPv6 Address (Octet 2) (Present if V6 FLAG is set)            |
        16      | IPv6 Address (Octet 3) (Present if V6 FLAG is set)            |
        17      | IPv6 Address (Octet 4) (Present if V6 FLAG is set)            |
        18      | IPv6 Address (Octet 5) (Present if V6 FLAG is set)            |
        19      | IPv6 Address (Octet 6) (Present if V6 FLAG is set)            |
        20      | IPv6 Address (Octet 7) (Present if V6 FLAG is set)            |
        21      | IPv6 Address (Octet 8) (Present if V6 FLAG is set)            |
        22      | IPv6 Address (Octet 9) (Present if V6 FLAG is set)            |
        23      | IPv6 Address (Octet 10) (Present if V6 FLAG is set)           |
        24      | IPv6 Address (Octet 11) (Present if V6 FLAG is set)           |
        25      | IPv6 Address (Octet 12) (Present if V6 FLAG is set)           |
        26      | IPv6 Address (Octet 13) (Present if V6 FLAG is set)           |
        27      | IPv6 Address (Octet 14) (Present if V6 FLAG is set)           |
        28      | IPv6 Address (Octet 15) (Present if V6 FLAG is set)           |
        29      | IPv6 Address (Octet 16) (Present if V6 FLAG is set)           |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub interface_type: InterfaceType,
    pub teid: u32,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address: Option<Ipv6Addr>,
}

impl InformationElement {
    pub fn new(
        interface_type: InterfaceType, 
        teid: u32, 
        ipv4_address: Option<Ipv4Addr>, 
        ipv6_address: Option<Ipv6Addr>, 
        instance: u8
    ) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    interface_type,
                    teid,
                    ipv4_address,
                    ipv6_address,
                    instance
                }
            )
        }
    }

    pub fn new_rand(
        interface_type: InterfaceType, 
        ipv4_address: Option<Ipv4Addr>, 
        ipv6_address: Option<Ipv6Addr>, 
        instance: u8
    ) -> Result<Self, String> {

        let random_teid = rand::random::<u32>();

        Self::new(
            interface_type, 
            random_teid,
            ipv4_address, 
            ipv6_address, 
            instance
        )

    }

    fn parse_flags(flags: u8) -> (u8, u8) {
        let v4 = (flags >> 7) & 0b1;
        let v6 = (flags >> 6) & 0b1;

        (v4, v6)
    }

    fn generate_flags(&self) -> u8 {
        let mut v4 = 0;
        let mut v6 = 0;

        if let Some(_) = self.ipv4_address {
            v4 = 1;
        }

        if let Some(_) = self.ipv6_address {
            v6 = 1;
        }

        ((v4 & 0b1) << 7) | ((v6 & 0b1) << 6)
    }

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
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

        let (v4, v6) = Self::parse_flags(buffer[pos]);
        let interface_type = buffer[pos] & 0xF;
        pos = pos + 1;

        let teid = NetworkEndian::read_u32(&buffer[pos..pos+4]);
        pos = pos + 4;

        let ipv4_address: Option<Ipv4Addr> = 
            if v4 == 1 {
                Some(Ipv4Addr::new(buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]))
            }
            else {
                None
            };

        if ipv4_address.is_some() {pos = pos + 4}

        let ipv6_address: Option<Ipv6Addr> = 
            if v6 == 1 {
                Some(Ipv6Addr::new(
                    NetworkEndian::read_u16(&buffer[pos..pos+2]),
                    NetworkEndian::read_u16(&buffer[pos+2..pos+4]),
                    NetworkEndian::read_u16(&buffer[pos+4..pos+6]),
                    NetworkEndian::read_u16(&buffer[pos+6..pos+8]),
                    NetworkEndian::read_u16(&buffer[pos+8..pos+10]),
                    NetworkEndian::read_u16(&buffer[pos+10..pos+12]),
                    NetworkEndian::read_u16(&buffer[pos+12..pos+14]),
                    NetworkEndian::read_u16(&buffer[pos+14..pos+16]),
                ))
            }
            else {
                None
            };

            if ipv6_address.is_some() {/*pos = pos + 16*/}

        if let Ok(interface_type) = InterfaceType::try_from(interface_type) {
            Some(
                (
                    InformationElement {
                        interface_type,
                        teid,
                        ipv4_address,
                        ipv6_address,
                        instance,
                    },
                    (length + 4) as usize
                )
            )
        }
        else {
            None
        }
        
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::FTEID
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

        let mut length = 4; // IE Headers

        length = length 
            + 1 // Flags and Interface type
            + 4; // TEID

        if let Some(_) = self.ipv4_address {
            length = length + 4
        }

        if let Some(_) = self.ipv6_address
        {
            length = length + 16
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

        buffer[pos] = self.generate_flags() | (self.interface_type as u8);
        pos = pos + 1;

        NetworkEndian::write_u32(&mut buffer[pos..pos+4], self.teid);
        pos = pos + 4;

        if let Some(a) = self.ipv4_address {
            for o in a.octets().iter() {
                buffer[pos] = *o;
                pos = pos + 1;
            }
        }

        if let Some(a) = self.ipv6_address {
            for o in a.octets().iter() {
                buffer[pos] = *o;
                pos = pos + 1;
            }
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
    fn test_new_rand() {
        let ie = InformationElement::new_rand(
            InterfaceType::S11MmeGtpC,
            Some(Ipv4Addr::new(10,0,0,1)),
            None,
            0
        );

        assert_eq!(ie.is_ok(), true);
    }

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let ie = InformationElement::new(
            InterfaceType::S11MmeGtpC,
            0x87654321,
            Some(Ipv4Addr::new(10,0,0,1)),
            None,
            0
        ).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::FTEID as u8,
            0, 9, // Length
            0, // Spare
            (0b1 << 7) | (0b0 << 6) | (InterfaceType::S11MmeGtpC as u8),
            0x87, 0x65, 0x43, 0x21,
            10, 0, 0, 1
        ]);
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(
            InterfaceType::S11MmeGtpC,
            0x87654321,
            Some(Ipv4Addr::new(10,0,0,1)),
            None,
            0
        ).unwrap();

        assert_eq!(ie.length(), 9+4);

        let ie = InformationElement::new(
            InterfaceType::S11MmeGtpC,
            0x87654321,
            Some(Ipv4Addr::new(10,0,0,1)),
            Some(Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE)),
            0
        ).unwrap();
            
        assert_eq!(ie.length(), 25+4);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(
            InterfaceType::S11MmeGtpC,
            0x87654321,
            Some(Ipv4Addr::new(10,0,0,1)),
            None,
            0
        ).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::FTEID as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::FTEID as u8,
            0, 25, // Length
            0, // Spare
            (0b1 << 7) | (0b1 << 6) | (InterfaceType::S11MmeGtpC as u8),
            0x87, 0x65, 0x43, 0x21,
            10, 0, 0, 1,
            0xFA, 0xDE, 
            0xDE, 0xAD, 
            0xBE, 0xEF, 
            0xCA, 0xFE,
            0xFE, 0xED,
            0xDE, 0xAF,
            0xBE, 0xAD,
            0xFA, 0xCE,
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.interface_type as u8, InterfaceType::S11MmeGtpC as u8);

            if let Some(a) = ie.ipv4_address {
                assert_eq!(a, Ipv4Addr::new(10,0,0,1));
            }
            else {
                assert!(false);
            }

            if let Some(a) = ie.ipv6_address {
                assert_eq!(a, Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE));
            }
            else {
                assert!(false);
            }
        }
        else {
            assert!(false);
        }
    }
}