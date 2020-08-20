/*
    |-----------------------------------------------------------------------------------|
    | Next Extension Header Field Value | Type of Extension Header                      |
    |-----------------------------------------------------------------------------------|
    | 0000 0000                         | No more extension headers                     |
    | 0000 0001                         | MBMS support indication                       |
    | 0000 0010                         | MS Info Change Reporting support indication   |
    | 0010 0000                         | Reserved for GTP-U. See 3GPP TS 29.281 [41].  |
    | 0100 0000                         | Reserved for GTP-U. See 3GPP TS 29.281 [41].  |
    | 1000 0001                         | Reserved for GTP-U. See 3GPP TS 29.281 [41].  |
    | 1100 0000                         | PDCP PDU number                               |
    | 1100 0001                         | Suspend Request                               |
    | 1100 0010                         | Suspend Response                              |
    |-----------------------------------------------------------------------------------|
*/

pub mod mbms_support_indication;
pub mod ms_info_change_reporting_support_indication;
pub mod pdcp_pdu_number;
pub mod suspend_request;
pub mod suspend_response;
pub mod udp_port;
pub mod long_pdcp_pdu_number;

#[derive(Copy, Clone, Debug)]
pub enum ExtensionHeaderType
{
    NoMore = 0b0000_0000,
    MbmsSi = 0b0000_0001,
    MsInfoChange = 0b0000_0010,
    // ServiceClassIndicator = 0b0010_0000, NOT IMPLEMENTED
    UDPPort = 0b0100_0000,
    // RANContainer = 0b1000_0001, NOT IMPLEMENTED
    LongPdcpPduNumber = 0b1000_0010,
    // XwRANContainer = 0b1000_0011, NOT IMPLEMENTED
    // NRRANContainer = 0b1000_0100, NOT IMPLEMENTED
    // PDUSessionContainer = 0b1000_0101, NOT IMPLEMENTED
    PdcpPduNum = 0b1100_0000,
    SuspendReq = 0b1100_0001,
    SuspendRes = 0b1100_0010
}

impl From<u8> for ExtensionHeaderType {
    fn from(v: u8) -> Self {
        match v {
            0b0000_0000 => ExtensionHeaderType::NoMore,
            0b0000_0001 => ExtensionHeaderType::MbmsSi,
            0b0000_0010 => ExtensionHeaderType::MsInfoChange,
            // 0b0010_0000 => ExtensionHeaderType::ServiceClassIndicator,
            0b0100_0000 => ExtensionHeaderType::UDPPort,
            // 0b1000_0001 => ExtensionHeaderType::RANContainer,
            // 0b1000_0010 => ExtensionHeaderType::LongPdcpPduNumber,
            // 0b1000_0011 => ExtensionHeaderType::XwRANContainer,
            // 0b1000_0100 => ExtensionHeaderType::NRRANContainer,
            // 0b1000_0101 => ExtensionHeaderType::PDUSessionContainer,
            0b1100_0000 => ExtensionHeaderType::PdcpPduNum,
            0b1100_0001 => ExtensionHeaderType::SuspendReq,
            0b1100_0010 => ExtensionHeaderType::SuspendRes,
            _ => panic!(format!("Unsupported Extension Header ({})", v))
        }
    }
}

pub trait ExtensionHeaderTraits {
    fn extension_header_type(&self) -> ExtensionHeaderType;
    fn set_next_extension_header_type(&mut self, next_extension_header_type: ExtensionHeaderType);
    fn next_extension_header_type(&self) -> ExtensionHeaderType;
    fn length(&self) -> u8;
    fn generate(&self, buffer: &mut[u8]) -> usize;
}

pub enum ExtensionHeader {
    LongPdcpPduNumber(long_pdcp_pdu_number::ExtensionHeader),
    MbmsSi(mbms_support_indication::ExtensionHeader),
    MsInfoChange(ms_info_change_reporting_support_indication::ExtensionHeader),
    PdcpPduNum(pdcp_pdu_number::ExtensionHeader),
    SuspendReq(suspend_request::ExtensionHeader),
    SuspendRes(suspend_response::ExtensionHeader),
    UDPPort(udp_port::ExtensionHeader)
}

impl ExtensionHeaderTraits for ExtensionHeader
{
    fn extension_header_type(&self) -> ExtensionHeaderType {
        match self {
            ExtensionHeader::LongPdcpPduNumber(eh) => eh.extension_header_type(),
            ExtensionHeader::MbmsSi(eh) => eh.extension_header_type(),
            ExtensionHeader::MsInfoChange(eh) => eh.extension_header_type(),
            ExtensionHeader::PdcpPduNum(eh) => eh.extension_header_type(),
            ExtensionHeader::SuspendReq(eh) => eh.extension_header_type(),
            ExtensionHeader::SuspendRes(eh) => eh.extension_header_type(),
            ExtensionHeader::UDPPort(eh) => eh.extension_header_type(),
        }
    }
    fn set_next_extension_header_type(&mut self, next_extension_header_type: ExtensionHeaderType) {
        match self {
            ExtensionHeader::LongPdcpPduNumber(eh) => eh.set_next_extension_header_type(next_extension_header_type),
            ExtensionHeader::MbmsSi(eh) => eh.set_next_extension_header_type(next_extension_header_type),
            ExtensionHeader::MsInfoChange(eh) => eh.set_next_extension_header_type(next_extension_header_type),
            ExtensionHeader::PdcpPduNum(eh) => eh.set_next_extension_header_type(next_extension_header_type),
            ExtensionHeader::SuspendReq(eh) => eh.set_next_extension_header_type(next_extension_header_type),
            ExtensionHeader::SuspendRes(eh) => eh.set_next_extension_header_type(next_extension_header_type),
            ExtensionHeader::UDPPort(eh) => eh.set_next_extension_header_type(next_extension_header_type),
        }
    }
    fn next_extension_header_type(&self) -> ExtensionHeaderType {
        match self {
            ExtensionHeader::LongPdcpPduNumber(eh) => eh.next_extension_header_type(),
            ExtensionHeader::MbmsSi(eh) => eh.next_extension_header_type(),
            ExtensionHeader::MsInfoChange(eh) => eh.next_extension_header_type(),
            ExtensionHeader::PdcpPduNum(eh) => eh.next_extension_header_type(),
            ExtensionHeader::SuspendReq(eh) => eh.next_extension_header_type(),
            ExtensionHeader::SuspendRes(eh) => eh.next_extension_header_type(),
            ExtensionHeader::UDPPort(eh) => eh.next_extension_header_type(),
        }
    }
    fn length(&self) -> u8 {
        match self {
            ExtensionHeader::LongPdcpPduNumber(eh) => eh.length(),
            ExtensionHeader::MbmsSi(eh) => eh.length(),
            ExtensionHeader::MsInfoChange(eh) => eh.length(),
            ExtensionHeader::PdcpPduNum(eh) => eh.length(),
            ExtensionHeader::SuspendReq(eh) => eh.length(),
            ExtensionHeader::SuspendRes(eh) => eh.length(),
            ExtensionHeader::UDPPort(eh) => eh.length(),
        }
    }
    fn generate(&self, buffer: &mut[u8]) -> usize {
        match self {
            ExtensionHeader::LongPdcpPduNumber(eh) => eh.generate(buffer),
            ExtensionHeader::MbmsSi(eh) => eh.generate(buffer),
            ExtensionHeader::MsInfoChange(eh) => eh.generate(buffer),
            ExtensionHeader::PdcpPduNum(eh) => eh.generate(buffer),
            ExtensionHeader::SuspendReq(eh) => eh.generate(buffer),
            ExtensionHeader::SuspendRes(eh) => eh.generate(buffer),
            ExtensionHeader::UDPPort(eh) => eh.generate(buffer),
        }
    }
}