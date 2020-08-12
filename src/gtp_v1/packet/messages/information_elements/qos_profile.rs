use byteorder::{ByteOrder, NetworkEndian};

use crate::field::*;

use super::{InformationElementTraits, InformationElementType};

pub const LENGTH: Field = 1..3;
pub const IPV4: Field = 3..7;
pub const IPV6: Field = 3..13;

#[derive(Copy, Clone, Debug)]
pub enum ReliabilityClass {
    SubscribedOrReserved = 0,
    Unused = 1,
    UnAckGTPAckLLCAckRLCProtectedData = 2,
    UnAckGTPUnAckLLCAckRLCProtectedData = 3,
    UnAckGTPUnAckLLCUnAckRLCProtectedData = 4,
    UnAckGTPUnAckLLCUnAckRLCUnProtectedData = 5,
    Reserved = 7
}

#[derive(Copy, Clone, Debug)]
pub enum DelayClass {
    SubscribedOrReserved = 0,
    DelayClass1 = 1,
    DelayClass2 = 2,
    DelayClass3 = 3,
    BestEffort = 4,
    Reserved = 7,
}

#[derive(Copy, Clone, Debug)]
pub enum PrecedenceClass {
    SubscribedOrReserved = 0,
    HighPriority = 1,
    NormalPriority = 2,
    LowPriority = 3,
    Reserved = 7,
}

#[derive(Copy, Clone, Debug)]
pub enum PeakThroughput {
    SubscribedOrReserved = 0,
    UpTo1000OctetsPerSecond = 1,
    UpTo2000OctetsPerSecond = 2,
    UpTo4000OctetsPerSecond = 3,
    UpTo8000OctetsPerSecond = 4,
    UpTo16000OctetsPerSecond = 5,
    UpTo32000OctetsPerSecond = 6,
    UpTo64000OctetsPerSecond = 7,
    UpTo128000OctetsPerSecond = 8,
    UpTo256000OctetsPerSecond = 9,
    Reserved = 0xFF,
}

#[derive(Copy, Clone, Debug)]
pub enum MeanThroughput {
    SubscribedOrReserved = 0,
    OctetsPerHour100 = 1,
    OctetsPerHour200 = 2,
    OctetsPerHour500 = 3,
    OctetsPerHour1000 = 4,
    OctetsPerHour2000 = 5,
    OctetsPerHour5000 = 6,
    OctetsPerHour10000 = 7,
    OctetsPerHour20000 = 8,
    OctetsPerHour50000 = 9,
    OctetsPerHour100000 = 10,
    OctetsPerHour200000 = 11,
    OctetsPerHour500000 = 12,
    OctetsPerHour1000000 = 13,
    OctetsPerHour2000000 = 14,
    OctetsPerHour5000000 = 15,
    OctetsPerHour10000000 = 16,
    OctetsPerHour20000000 = 17, 
    OctetsPerHour50000000 = 18,
    Reserved = 0x1E,
    BestEffort = 0x1F,
}

#[derive(Copy, Clone, Debug)]
pub enum DeliveryOfErroneusSDUs {
    SubscribedOrReserved = 0,
    NoDetect = 1,
    Yes = 2,
    No = 3,
    Reserved = 7
}

#[derive(Copy, Clone, Debug)]
pub enum DeliveryOrder {
    SubscribedOrReserved = 0,
    NoDetect = 1,
    Yes = 2,
    No = 3,
    Reserved = 7
}

#[derive(Copy, Clone, Debug)]
pub enum TrafficClass {
    SubscribedOrReserved = 0,
    Conversational = 1,
    Streaming = 2,
    Interactive = 3,
    Background = 4,
    Reserved = 7
}

#[derive(Copy, Clone, Debug)]
pub enum ResidualBitErrorRate {
    SubscribedOrReserved = 0,
    BER5x10POWn2 = 1,
    BER1x10POWn2 = 2,
    BER5x10POWn3 = 3,
    BER4x10POWn3 = 4,
    BER1x10POWn3 = 5,
    BER1x10POWn4 = 6,
    BER1x10POWn5 = 7,
    BER1x10POWn6 = 8,
    BER1x10POWn8 = 9,
    Reserved = 0xFF,
}

#[derive(Copy, Clone, Debug)]
pub enum SDUErrorRatio {
    SubscribedOrReserved = 0,
    SDUER1x10POWn2 = 1,
    SDUER7x10POWn3 = 2,
    SDUER1x10POWn3 = 3,
    SDUER1x10POWn4 = 4,
    SDUER1x10POWn5 = 5,
    SDUER1x10POWn6 = 6,
    SDUER1x10POWn1 = 7,
    Reserved = 0xFF,
}

#[derive(Copy, Clone, Debug)]
pub enum TrafficHandlingPriority {
    SubscribedOrReserved = 0,
    PriorityLevel1 = 1,
    PriorityLevel2 = 2,
    PriorityLevel3 = 3,
}

#[derive(Copy, Clone, Debug)]
pub enum SourceStatisticsDescriptor {
    UnknownOrSpare = 0,
    Speech = 1,
}

#[derive(Copy, Clone, Debug)]
pub enum SignallingIndication {
    NotOptimisedForSignallingTraffic = 0,
    OptimisedForSignallingTraffic = 1,
}

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (135)                                                 |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Allocation/Retention Priority                                 |
        5       | 0     | 0     |  Delay Class          | Reliability Class     |
        6       | Peak Throughput               | 0     | Precedence Class      |
        7       | 0     | 0     | 0     | Mean Throughput                       |
        8       | Traffic Class         | Delivery Order| Delivery of E SDU     |
        9       | Maximum SDU Size                                              |
        11      | Maximum Bitrate for Uplink                                    |
        12      | Maximum Bitrate for Downlink                                  |
        13      | Residual BER                  | SDU Error Ratio               |
        14      | Transfer Delay                                | THP           |
        15      | Guaranteed Bitrate for Uplink                                 |
        16      | Guaranteed Bitrate for Downlink                               |
        17      | 0     | 0     | 0     | SI    | Source Statistics Descriptor  |
        18      | Maximum Bitrate for Downlink (extended)                       |
        19      | Guaranteed Bitrate for Downlink (extended)                    |
        20      | Maximum Bitrate for Uplink (extended)                         |
        21      | Guaranteed Bitrate for Uplink (extended)                      |
        22      | Maximum Bitrate for Downlink (extended-2)                     |
        23      | Guaranteed Bitrate for Downlink (extended-2)                  |
        24      | Maximum Bitrate for Uplink (extended-2)                       |
        25      | Guaranteed Bitrate for Uplink (extended-2)                    |
                |---------------------------------------------------------------|

        NOTE: Not all fields are mandatory (only octets 1 - 7 are required)
    */

    arp: u8,
    delay_class: DelayClass,
    reliability_class: ReliabilityClass,
    peak_throughput: PeakThroughput,
    precedence_class: PrecedenceClass,
    mean_throughput: MeanThroughput,
    traffic_class: Option<TrafficClass>,
    delivery_order: Option<DeliveryOrder>,
    delivery_of_erroneus_sdus: Option<DeliveryOfErroneusSDUs>,
    maximum_sdu_size: Option<u8>,
    maximum_uplink_bitrate: Option<u32>,
    maximum_downlink_bitrate: Option<u32>,
    residual_ber: Option<ResidualBitErrorRate>,
    sdu_error_ratio: Option<SDUErrorRatio>,
    transfer_delay: Option<u32>,
    traffic_handling_priority: Option<TrafficHandlingPriority>,
    guaranteed_uplink_bitrate: Option<u32>,
    guaranteed_downlink_bitrate: Option<u32>,
}

impl InformationElement {
    pub fn new(arp: u8, 
            delay_class: DelayClass, 
            reliability_class: ReliabilityClass,
            peak_throughput: PeakThroughput,
            precedence_class: PrecedenceClass,
            mean_throughput: MeanThroughput,
        ) -> Self {
        InformationElement {
            arp,
            delay_class,
            reliability_class,
            peak_throughput,
            precedence_class,
            mean_throughput,
            traffic_class: None,
            delivery_order: None,
            delivery_of_erroneus_sdus: None,
            maximum_sdu_size: None,
            maximum_uplink_bitrate: None,
            maximum_downlink_bitrate: None,
            residual_ber: None,
            sdu_error_ratio: None,
            transfer_delay: None,
            traffic_handling_priority: None,
            guaranteed_uplink_bitrate: None,
            guaranteed_downlink_bitrate: None,
        }
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::QoSProfile
    }

    fn length(&self) -> u16 {
        4+3
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

        // Write ARP
        buffer[end] = self.arp;
        end = end + 1;

        // Write Delay Class and Reliability Class
        buffer[end] = (self.delay_class as u8) << 3 | (self.reliability_class as u8);
        end = end + 1;

        // Write Peak Throughput and Precedence Class
        buffer[end] = (self.peak_throughput as u8) << 4 | (self.precedence_class as u8);
        end = end + 1;

        // Write Mean Throughput
        buffer[end] = self.mean_throughput as u8;
        end = end + 1;

        // TODO: Generate the rest of the optional fields

        end
    }
    
    fn parse(&mut self, _buffer: &[u8]) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MTU;
    use crate::gtp_v1::packet::messages::information_elements::InformationElementType;

    #[test]
    fn test_generate() {
        let mut buffer = [0; MTU];

        let arp = 8;
        let delay_class = DelayClass::BestEffort;
        let reliability_class = ReliabilityClass::UnAckGTPUnAckLLCUnAckRLCUnProtectedData;
        let peak_throughput = PeakThroughput::UpTo1000OctetsPerSecond;
        let precedence_class = PrecedenceClass::NormalPriority;
        let mean_throughput = MeanThroughput::BestEffort;

        let ie = InformationElement::new(
            arp,
            delay_class,
            reliability_class,
            peak_throughput,
            precedence_class,
            mean_throughput
        );

        let end = ie.generate(&mut buffer);

        assert_eq!(buffer[..end], 
            [
                InformationElementType::QoSProfile as u8,
                0, 4,
                8, // ARP
                0b0010_0101, // Delay Class and Reliability Class
                0b0001_0010, // Peak Throughput and Precedence Class
                0x1F
        ]);
    }
    
    #[test]
    fn test_length() {
        let arp = 8;
        let delay_class = DelayClass::BestEffort;
        let reliability_class = ReliabilityClass::UnAckGTPUnAckLLCUnAckRLCUnProtectedData;
        let peak_throughput = PeakThroughput::UpTo1000OctetsPerSecond;
        let precedence_class = PrecedenceClass::NormalPriority;
        let mean_throughput = MeanThroughput::BestEffort;

        let ie = InformationElement::new(
            arp,
            delay_class,
            reliability_class,
            peak_throughput,
            precedence_class,
            mean_throughput
        );

        assert_eq!(ie.length(), 7);
    }
    #[test]
    fn test_message_type() {
        let arp = 8;
        let delay_class = DelayClass::BestEffort;
        let reliability_class = ReliabilityClass::UnAckGTPUnAckLLCUnAckRLCUnProtectedData;
        let peak_throughput = PeakThroughput::UpTo1000OctetsPerSecond;
        let precedence_class = PrecedenceClass::NormalPriority;
        let mean_throughput = MeanThroughput::BestEffort;

        let ie = InformationElement::new(
            arp,
            delay_class,
            reliability_class,
            peak_throughput,
            precedence_class,
            mean_throughput
        );

        assert_eq!(ie.information_element_type() as u8, InformationElementType::QoSProfile as u8)
    }

    #[test]
    fn test_message_parse() {
        assert_eq!(1, 1)
    }
}