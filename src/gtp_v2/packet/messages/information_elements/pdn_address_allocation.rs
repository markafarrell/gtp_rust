use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

use super::pdn_type::PDNType;

use std::net::{Ipv4Addr, Ipv6Addr};

use std::convert::TryFrom;

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (79)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       |                                       | PDN Type              |
        6       | IPv6 Address Prefix Length (Present if PDN type = IPv6 or IPv4v6) |
        7       | IPv6 Address (Octet 1) (Present if PDN type = IPv6 or IPv4v6) |
        8       | IPv6 Address (Octet 2) (Present if PDN type = IPv6 or IPv4v6) |
        9       | IPv6 Address (Octet 3) (Present if PDN type = IPv6 or IPv4v6) |
        10      | IPv6 Address (Octet 4) (Present if PDN type = IPv6 or IPv4v6) |
        11      | IPv6 Address (Octet 5) (Present if PDN type = IPv6 or IPv4v6) |
        12      | IPv6 Address (Octet 6) (Present if PDN type = IPv6 or IPv4v6) |
        13      | IPv6 Address (Octet 7) (Present if PDN type = IPv6 or IPv4v6) |
        14      | IPv6 Address (Octet 8) (Present if PDN type = IPv6 or IPv4v6) |
        15      | IPv6 Address (Octet 9) (Present if PDN type = IPv6 or IPv4v6) |
        16      | IPv6 Address (Octet 10) (Present if PDN type = IPv6 or IPv4v6)|
        17      | IPv6 Address (Octet 11) (Present if PDN type = IPv6 or IPv4v6)|
        18      | IPv6 Address (Octet 12) (Present if PDN type = IPv6 or IPv4v6)|
        19      | IPv6 Address (Octet 13) (Present if PDN type = IPv6 or IPv4v6)|
        20      | IPv6 Address (Octet 14) (Present if PDN type = IPv6 or IPv4v6)|
        21      | IPv6 Address (Octet 15) (Present if PDN type = IPv6 or IPv4v6)|
        22      | IPv6 Address (Octet 16) (Present if PDN type = IPv6 or IPv4v6)|
        23      | IPv4 Address (Octet 1) (Present if PDN type = IPv4 or IPv4v6) |
        24      | IPv4 Address (Octet 2) (Present if PDN type = IPv4 or IPv4v6) |
        25      | IPv4 Address (Octet 3) (Present if PDN type = IPv4 or IPv4v6) |
        26      | IPv4 Address (Octet 4) (Present if PDN type = IPv4 or IPv4v6) |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub pdn_type: PDNType,
    pub ipv4_address: Option<Ipv4Addr>,
    pub ipv6_address_and_prefix: Option<(Ipv6Addr, u8)>,
}

impl InformationElement {
    pub fn new(
        pdn_type: PDNType, 
        ipv4_address: Option<Ipv4Addr>,
        ipv6_address_and_prefix: Option<(Ipv6Addr, u8)>, 
        instance: u8
    ) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else if ( pdn_type == PDNType::IPv4 || pdn_type == PDNType::IPv4v6) && ipv4_address.is_none() {
            Err("ipv4_address must be defined when pdn_type is IPv4 or IPv4v6.".to_string())
        }
        else if ( pdn_type == PDNType::IPv6 || pdn_type == PDNType::IPv4v6) && ipv6_address_and_prefix.is_none() {
            Err("ipv6_address must be defined when pdn_type is IPv6 or IPv4v6.".to_string())
        }
        else if pdn_type == PDNType::NonIp && ( ipv6_address_and_prefix.is_some() || ipv4_address.is_some() ) {
            Err("ipv6_address or ipv4_address must NOT be defined when pdn_type is NonIp.".to_string())
        }
        else {
            Ok(
                InformationElement {
                    pdn_type,
                    ipv4_address,
                    ipv6_address_and_prefix,
                    instance: instance,
                }
            )
        }
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

        if let Ok(pdn_type) = PDNType::try_from(buffer[pos]) {
            pos = pos + 1;

            let ipv6_address_and_prefix: Option<(Ipv6Addr, u8)> = 
                if pdn_type == PDNType::IPv6 || pdn_type == PDNType::IPv4v6 {
                    let prefix = buffer[pos];
                    pos = pos + 1;

                    Some(
                        (
                            Ipv6Addr::new(
                                NetworkEndian::read_u16(&buffer[pos..pos+2]),
                                NetworkEndian::read_u16(&buffer[pos+2..pos+4]),
                                NetworkEndian::read_u16(&buffer[pos+4..pos+6]),
                                NetworkEndian::read_u16(&buffer[pos+6..pos+8]),
                                NetworkEndian::read_u16(&buffer[pos+8..pos+10]),
                                NetworkEndian::read_u16(&buffer[pos+10..pos+12]),
                                NetworkEndian::read_u16(&buffer[pos+12..pos+14]),
                                NetworkEndian::read_u16(&buffer[pos+14..pos+16]),
                            ),
                            prefix
                        )
                    )
                }
                else {
                    None
                };

            if ipv6_address_and_prefix.is_some() {pos = pos + 16}

            let ipv4_address: Option<Ipv4Addr> = 
            if pdn_type == PDNType::IPv4 || pdn_type == PDNType::IPv4v6 {
                Some(Ipv4Addr::new(buffer[pos], buffer[pos+1], buffer[pos+2], buffer[pos+3]))
            }
            else {
                None
            };

        if ipv4_address.is_some() {/*pos = pos + 4*/}
            Some(
                (
                    InformationElement {
                        pdn_type,
                        ipv4_address,
                        ipv6_address_and_prefix,
                        instance: instance,
                    },
                    (length + 4) as usize
                )
            )
        }
        else { None }
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::PDNAddressAllocation
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
            + 1; // PDN Type

        if let Some(_) = self.ipv6_address_and_prefix
        {
            length = length + 17
        }

        if let Some(_) = self.ipv4_address {
            length = length + 4
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

        buffer[pos] = (self.pdn_type as u8) & 0b111;
        pos = pos + 1;

        if let Some((a, prefix)) = self.ipv6_address_and_prefix {
            buffer[pos] = prefix;
            pos = pos + 1;
            for o in a.octets().iter() {
                buffer[pos] = *o;
                pos = pos + 1;
            }
        }

        if let Some(a) = self.ipv4_address {
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
    fn test_generate() {
        let mut buffer = [0; MTU];

        let ie = InformationElement::new(PDNType::IPv4, None, None, 0);

        // We didn't specify the IPv4 Address
        assert_eq!(ie.is_ok(), false);

        let ie = InformationElement::new(PDNType::IPv6, None, None, 0);

        // We didn't specify the IPv6 Address
        assert_eq!(ie.is_ok(), false);

        let ie = InformationElement::new(
            PDNType::IPv4v6, 
            None, 
            Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)), 
            0);

        // We didn't specify the IPv4 Address
        assert_eq!(ie.is_ok(), false);

        let ie = InformationElement::new(
            PDNType::IPv4v6, 
            Some(Ipv4Addr::new(10,0,0,1)), 
            None, 
            0);

        // We didn't specify the IPv6 Address
        assert_eq!(ie.is_ok(), false);

        let ie = InformationElement::new(PDNType::IPv4, Some(Ipv4Addr::new(10,0,0,1)), None, 0);

        assert_eq!(ie.is_ok(), true);

        let pos = ie.unwrap().generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::PDNAddressAllocation as u8,
            0, 5, // Length
            0, // Spare
            PDNType::IPv4 as u8, // PDN Type
            10,0,0,1,
        ]);

        let ie = InformationElement::new(PDNType::IPv6, None, Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)), 0);

        assert_eq!(ie.is_ok(), true);

        let pos = ie.unwrap().generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::PDNAddressAllocation as u8,
            0, 18, // Length
            0, // Spare
            PDNType::IPv6 as u8, // PDN Type
            128, // Prefix
            0xFA, 0xDE, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xFE, 0xED, 0xDE, 0xAF, 0xBE, 0xAD, 0xFA, 0xCE, // IPv6 Address
        ]);

        let ie = InformationElement::new(PDNType::IPv4v6, Some(Ipv4Addr::new(10,0,0,1)), Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)), 0);

        assert_eq!(ie.is_ok(), true);

        let pos = ie.unwrap().generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::PDNAddressAllocation as u8,
            0, 22, // Length
            0, // Spare
            PDNType::IPv4v6 as u8, // PDN Type
            128, // Prefix
            0xFA, 0xDE, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xFE, 0xED, 0xDE, 0xAF, 0xBE, 0xAD, 0xFA, 0xCE, // IPv6 Address
            10,0,0,1,
        ]);

        let ie = InformationElement::new(PDNType::NonIp, None, None, 0);

        assert_eq!(ie.is_ok(), true);

        let pos = ie.unwrap().generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::PDNAddressAllocation as u8,
            0, 1, // Length
            0, // Spare
            PDNType::NonIp as u8, // PDN Type
        ]);
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(
            PDNType::IPv4v6, 
            Some(Ipv4Addr::new(10,0,0,1)), 
            Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)),
            0).unwrap();
        assert_eq!(ie.length(), 26);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(
            PDNType::IPv4v6, 
            Some(Ipv4Addr::new(10,0,0,1)), 
            Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)),
            0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::PDNAddressAllocation as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::PDNAddressAllocation as u8,
        0, 17, // Length
        0, // Spare
        PDNType::IPv4v6 as u8, // PDN Type
        128, // Prefix
        0xFA, 0xDE, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xFE, 0xED, 0xDE, 0xAF, 0xBE, 0xAD, 0xFA, 0xCE, // IPv6 Address
        10,0,0,1, // IPv4 Address
    ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.pdn_type, PDNType::IPv4v6);
            assert_eq!(ie.ipv6_address_and_prefix,Some((Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE), 128)));
            assert_eq!(ie.ipv4_address,Some(Ipv4Addr::new(10,0,0,1)));
        }
        else {
            assert!(false);
        }
    }
}