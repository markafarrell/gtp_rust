use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (73)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       | Spare | PCI   | PL                            | Spare | PVI   |
        6       | Label (QCI)                                                   |
        7       | Maximum bitrate for Uplink (Octet 1)                          |
        8       | Maximum bitrate for Uplink (Octet 2)                          |
        9       | Maximum bitrate for Uplink (Octet 3)                          |
        10      | Maximum bitrate for Uplink (Octet 4)                          |
        11      | Maximum bitrate for Uplink (Octet 5)                          |
        12      | Maximum bitrate for Downlink (Octet 1)                        |
        13      | Maximum bitrate for Downlink (Octet 2)                        |
        14      | Maximum bitrate for Downlink (Octet 3)                        |
        15      | Maximum bitrate for Downlink (Octet 4)                        |
        16      | Maximum bitrate for Downlink (Octet 5)                        |
        17      | Guaranteed bitrate for Uplink (Octet 1)                       |
        18      | Guaranteed bitrate for Uplink (Octet 2)                       |
        19      | Guaranteed bitrate for Uplink (Octet 3)                       |
        20      | Guaranteed bitrate for Uplink (Octet 4)                       |
        21      | Guaranteed bitrate for Uplink (Octet 5)                       |
        22      | Guaranteed bitrate for Downlink (Octet 1)                     |
        23      | Guaranteed bitrate for Downlink (Octet 2)                     |
        24      | Guaranteed bitrate for Downlink (Octet 3)                     |
        25      | Guaranteed bitrate for Downlink (Octet 4)                     |
        26      | Guaranteed bitrate for Downlink (Octet 5)                     |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub pci: bool,
    pl: u8,
    pub pvi: bool,
    pub qci: u8,
    max_ul_bitrate: u64,
    max_dl_bitrate: u64,
    guaranteed_ul_bitrate: u64,
    guaranteed_dl_bitrate: u64,
}

impl InformationElement {
    pub fn new(
        pci: bool,
        pl: u8,
        pvi: bool,
        qci: u8,
        max_ul_bitrate: u64,
        max_dl_bitrate: u64,
        guaranteed_ul_bitrate: u64,
        guaranteed_dl_bitrate: u64,
        instance: u8,
    ) -> Result<Self, String> {

        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else if pl > 0xF {
            Err(format!("PL is > 0xF {}", pl))
        }
        else if max_ul_bitrate > 10_000_000 {
            Err(format!("Max UL Bitrate is > 10,000,000 {}", max_ul_bitrate))
        }
        else if max_dl_bitrate > 10_000_000 {
            Err(format!("Max DL Bitrate is > 10,000,000 {}", max_dl_bitrate))
        }
        else if guaranteed_ul_bitrate > 10_000_000 {
            Err(format!("Guaranteed UL Bitrate is > 10,000,000 {}", guaranteed_ul_bitrate))
        }
        else if guaranteed_dl_bitrate > 10_000_000 {
            Err(format!("Guaranteed DL Bitrate is > 10,000,000 {}", guaranteed_dl_bitrate))
        }
        else {
            Ok(InformationElement {
                pci,
                pl,
                pvi,
                qci,
                max_ul_bitrate,
                max_dl_bitrate,
                guaranteed_ul_bitrate,
                guaranteed_dl_bitrate,
                instance: instance,
            })
        }
    }

    fn generate_flags(&self) -> u8 {
        (if self.pci {0} else {1} << 6) | 
        ((self.pl & 0xF) << 2) | 
        (if self.pvi {0} else {1})
    }

    fn parse_flags(flags: u8) -> (bool, u8, bool) {
        let pci = ((flags >> 6) & 0b1) == 1;
        let pl = (flags >> 2) & 0xF;
        let pvi = ((flags) & 0b1) == 1;

        (pci, pl, pvi)
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

        let (pci, pl, pvi) = Self::parse_flags(buffer[pos]);
        pos = pos + 1;

        let qci = buffer[pos];
        pos = pos + 1;

        let max_ul_bitrate = NetworkEndian::read_uint(&buffer[pos..pos+5], 5);
        pos = pos + 5;

        let max_dl_bitrate = NetworkEndian::read_uint(&buffer[pos..pos+5], 5);
        pos = pos + 5;

        let guaranteed_ul_bitrate = NetworkEndian::read_uint(&buffer[pos..pos+5], 5);
        pos = pos + 5;

        let guaranteed_dl_bitrate = NetworkEndian::read_uint(&buffer[pos..pos+5], 5);
        // pos = pos + 5;

        Some(
            (
                InformationElement {
                    pci,
                    pl,
                    pvi,
                    qci,
                    max_ul_bitrate,
                    max_dl_bitrate,
                    guaranteed_ul_bitrate,
                    guaranteed_dl_bitrate,
                    instance: instance,
                },
                (length + 4) as usize
            )
        )        
    }

    pub fn pl(&self) -> u8 {
        self.pl
    }
    pub fn max_ul_bitrate(&self) -> u64 {
        self.max_ul_bitrate
    }
    pub fn max_dl_bitrate(&self) -> u64 {
        self.max_dl_bitrate
    }
    pub fn guaranteed_ul_bitrate(&self) -> u64 {
        self.guaranteed_ul_bitrate
    }
    pub fn guaranteed_dl_bitrate(&self) -> u64 {
        self.guaranteed_dl_bitrate
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::BearerQoS
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

        4+22
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

        buffer[pos] = self.generate_flags();
        pos = pos + 1;

        buffer[pos] = self.qci;
        pos = pos + 1;

        NetworkEndian::write_uint(&mut buffer[pos..pos+6], self.max_ul_bitrate, 5);
        pos = pos + 5;

        NetworkEndian::write_uint(&mut buffer[pos..pos+6], self.max_dl_bitrate, 5);
        pos = pos + 5;

        NetworkEndian::write_uint(&mut buffer[pos..pos+6], self.guaranteed_ul_bitrate, 5);
        pos = pos + 5;

        NetworkEndian::write_uint(&mut buffer[pos..pos+6], self.guaranteed_dl_bitrate, 5);
        pos = pos + 5;

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

        if let Ok(ie) = InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_000,
            0,
            0,
            0
        ) {
            let pos = ie.generate(&mut buffer);

            assert_eq!(buffer[..pos], [InformationElementType::BearerQoS as u8,
                0, 22, // Length
                0, // Spare
                0b01100100, // Flags
                7, // QCI
                0x00, 0x00, 0x98, 0x96, 0x80,
                0x00, 0x00, 0x98, 0x96, 0x80,
                0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00,
            ]);
        }
        else {
            assert!(false);
        }

        if let Ok(_) = InformationElement::new(
            false,
            0x1F,
            true,
            7,
            10_000_000,
            10_000_000,
            0,
            0,
            0
        ) {
            // This should fail PL must be less than 0xF
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(_) = InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_001,
            10_000_000,
            10_000_000,
            10_000_000,
            0
        ) {
            // This should fail Max UL Bitrate must be less than 10,000,000
            assert!(false);
        }
        else {
            assert!(true);
        }

        if let Ok(_) = InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_001,
            10_000_000,
            10_000_000,
            0
        ) {
            // This should fail Max DL Bitrate must be less than 10,000,000
            assert!(false);
        }
        else {
            assert!(true);
        }
        if let Ok(_) = InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_000,
            10_000_001,
            10_000_000,
            0
        ) {
            // This should fail Guaranteed UL Bitrate must be less than 10,000,000
            assert!(false);
        }
        else {
            assert!(true);
        }
        if let Ok(_) = InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_000,
            10_000_000,
            10_000_001,
            0
        ) {
            // This should fail Guaranteed DL Bitrate must be less than 10,000,000
            assert!(false);
        }
        else {
            assert!(true);
        }
    }

    #[test]
    fn test_length() {
        if let Ok(ie) = InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_000,
            0,
            0,
            0
        ) {
            assert_eq!(ie.length(), 22+4);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn test_message_type() {
        if let Ok(ie) = InformationElement::new(
            false,
            9,
            true,
            7,
            10_000_000,
            10_000_000,
            0,
            0,
            0
        ) {
            assert_eq!(ie.information_element_type() as u8, InformationElementType::BearerQoS as u8);
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::BearerQoS as u8,
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
            assert_eq!(ie.pci, false);
            assert_eq!(ie.pvi, true);
            assert_eq!(ie.qci, 7);
            assert_eq!(ie.pl(), 9);
            assert_eq!(ie.max_ul_bitrate(), 10_000_000);
            assert_eq!(ie.max_dl_bitrate(), 10_000_000);
            assert_eq!(ie.guaranteed_ul_bitrate(), 0);
            assert_eq!(ie.guaranteed_dl_bitrate(), 0);
        }
        else {
            assert!(false);
        }
    }
}