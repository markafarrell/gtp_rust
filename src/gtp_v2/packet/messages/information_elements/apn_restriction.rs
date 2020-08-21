use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MaximumAPNRestrictionValue {
    Unrestricted = 0,
    Public1 = 1,
    Public2 = 2,
    Private1 = 3,
    Private2 = 4,
}

impl TryFrom<u8> for MaximumAPNRestrictionValue
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MaximumAPNRestrictionValue::Unrestricted),
            1 => Ok(MaximumAPNRestrictionValue::Public1),
            2 => Ok(MaximumAPNRestrictionValue::Public2),
            3 => Ok(MaximumAPNRestrictionValue::Private1),
            4 => Ok(MaximumAPNRestrictionValue::Private2),
            _ => Err(format!("Unsupported Maximum APN Restriction Value type ({})", value))
        }
    }
}

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (82)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       | Maximum APN Restriction Value                                 |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub maximum_apn_restriction: MaximumAPNRestrictionValue
}

impl InformationElement {
    pub fn new(maximum_apn_restriction: MaximumAPNRestrictionValue, instance: u8) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    maximum_apn_restriction,
                    instance,
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

        if let Ok(maximum_apn_restriction) = MaximumAPNRestrictionValue::try_from(buffer[pos]){
            // pos = pos + 1;

            Some(
                (
                    InformationElement {
                        maximum_apn_restriction,
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
        InformationElementType::APNRestriction
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

        4+1
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

        buffer[pos] = self.maximum_apn_restriction as u8;
        pos = pos + 1;

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

        let ie = InformationElement::new(MaximumAPNRestrictionValue::Private2, 0).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::APNRestriction as u8,
            0, 1, // Length
            0, // Spare
            4, // APN Restriction
        ]);
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(MaximumAPNRestrictionValue::Private2, 0).unwrap();
        assert_eq!(ie.length(), 5);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(MaximumAPNRestrictionValue::Private2, 0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::APNRestriction as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::APNRestriction as u8,
            0, 1, // Length
            0, // Spare
            4, // APN Restriction
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.maximum_apn_restriction, MaximumAPNRestrictionValue::Private2);
        }
        else {
            assert!(false);
        }
    }
}