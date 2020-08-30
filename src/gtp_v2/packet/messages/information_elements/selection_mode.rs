use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SelectionMode {
    MSorNetworkProvidedAPNSubscriptionVerified = 0,
    MSProvidedAPNSubscriptionNotVerified = 1,
    NetworkProvidedAPNSubscriptionNotVerified = 2,
    ForFutureUse = 3,
}

impl TryFrom<u8> for SelectionMode
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified),
            1 => Ok(SelectionMode::MSProvidedAPNSubscriptionNotVerified),
            2 => Ok(SelectionMode::NetworkProvidedAPNSubscriptionNotVerified),
            3 => Ok(SelectionMode::ForFutureUse),
            _ => Err(format!("Unsupported Selection Mode ({})", value))
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
        5       |                                               | Selection Mode|
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub selection_mode: SelectionMode
}

impl InformationElement {
    pub fn new(selection_mode: SelectionMode, instance: u8) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    selection_mode,
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

        if let Ok(selection_mode) = SelectionMode::try_from(buffer[pos]){
            // pos = pos + 1;

            Some(
                (
                    InformationElement {
                        selection_mode,
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
        InformationElementType::SelectionMode
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

        buffer[pos] = self.selection_mode as u8;
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

        let ie = InformationElement::new(SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified, 0).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::SelectionMode as u8,
            0, 1, // Length
            0, // Spare
            SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified as u8, // Selection Mode
        ]);
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified, 0).unwrap();
        assert_eq!(ie.length(), 5);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified, 0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::SelectionMode as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::RATType as u8,
            0, 1, // Length
            0, // Spare
            SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified as u8, // Selection Mode
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.selection_mode, SelectionMode::MSorNetworkProvidedAPNSubscriptionVerified);
        }
        else {
            assert!(false);
        }
    }
}