use std::net::IpAddr;
use byteorder::{ByteOrder, NetworkEndian};

use crate::field::*;

use super::{InformationElementTraits, InformationElementType};

pub const LENGTH: Field = 1..3;
pub const IPV4: Field = 3..7;
pub const IPV6: Field = 3..13;

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (133)                                                 |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4->n    | GSN Address                                                   |
                |---------------------------------------------------------------|
    */

    gsn_address: IpAddr
}

impl InformationElement {
    pub fn new(address: IpAddr) -> Self {
        InformationElement {
            gsn_address: address
        }
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::GsnAddress
    }

    fn length(&self) -> u16 {
        match self.gsn_address
        {
            IpAddr::V4(_) => 3+4,
            IpAddr::V6(_) => 3+16,
        }
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut end = 0;
        
        // Write the type
        buffer[end] = self.information_element_type() as u8;

        end = end + 1;

        // Write the length
        // We subtract 3 octets as the type and length fields aren't included.
        NetworkEndian::write_u16(&mut buffer[LENGTH],self.length()-3);
        end = end + 2;

        match self.gsn_address
        {
            IpAddr::V4(a) => {
                for o in a.octets().iter() {
                    buffer[end] = *o;
                    end = end + 1;
                }
            },
            IpAddr::V6(a) => {
                for o in a.octets().iter() {
                    buffer[end] = *o;
                    end = end + 1;
                }
            }
        };

        end
    }
    
    fn parse(&mut self, _buffer: &[u8]) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
    use crate::MTU;
    use crate::gtp_v1::packet::messages::information_elements::InformationElementType;

    #[test]
    fn test_generate_ipv4() {
        let mut buffer = [0; MTU];

        let address = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));

        let ie = InformationElement::new(address);

        let end = ie.generate(&mut buffer);

        assert_eq!(buffer[..end], [InformationElementType::GsnAddress as u8,
            0, 4, 
            192, 168, 0, 1
        ]);
    }
    
    #[test]
    fn test_generate_ipv6() {
        let mut buffer = [0; MTU];

        let address = IpAddr::V6(Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE));

        let ie = InformationElement::new(address);

        let end = ie.generate(&mut buffer);

        assert_eq!(buffer[..end], [InformationElementType::GsnAddress as u8, 
            0, 16, 
            0xFA, 0xDE, 
            0xDE, 0xAD, 
            0xBE, 0xEF, 
            0xCA, 0xFE,
            0xFE, 0xED,
            0xDE, 0xAF,
            0xBE, 0xAD,
            0xFA, 0xCE,
        ]);
    }

    #[test]
    fn test_length_ipv4() {
        let address = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));

        let ie = InformationElement::new(address);
        assert_eq!(ie.length(), 7);
    }

    #[test]
    fn test_length_ipv6() {
        let address = IpAddr::V6(Ipv6Addr::new(0xFADE, 0xDEAD, 0xBEEF, 0xCAFE, 0xFEED, 0xDEAF, 0xBEAD, 0xFACE));

        let ie = InformationElement::new(address);
        assert_eq!(ie.length(), 19);
    }
    #[test]
    fn test_message_type() {
        let address = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1));

        let ie = InformationElement::new(address);

        assert_eq!(ie.information_element_type() as u8, InformationElementType::GsnAddress as u8)
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}