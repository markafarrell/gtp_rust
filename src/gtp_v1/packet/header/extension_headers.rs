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

pub trait ExtensionHeaderTraits {
    fn extension_header_type(&self) -> ExtensionHeaderType;
    fn set_next_extension_header_type(&mut self, next_extension_header_type: ExtensionHeaderType);
    fn next_extension_header_type(&self) -> ExtensionHeaderType;
    fn length(&self) -> u8;
    fn generate(&self, buffer: &mut[u8]) -> usize;
    fn parse(&mut self, buffer: &[u8]);
}
