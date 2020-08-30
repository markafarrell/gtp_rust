use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CauseSource {
    LocalNode = 0,
    RemoteNode = 1,
}

impl TryFrom<u8> for CauseSource
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CauseSource::LocalNode),
            1 => Ok(CauseSource::RemoteNode),
            _ => Err(format!("Unsupported Cause Source ({})", value))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CauseCode {
    // ReservedShallNotBeSent = 0,
    // Request / Initial Message
    Reserved = 1,
    LocalDetach = 2,
    CompleteDetach = 3,
    RATChangedfrom3GPPtoNon3GPP = 4,
    ISRDeactivation = 5,
    ErrorIndicationReceivedfromRNCeNodeBS4SGSNMME = 6,
    IMSIDetachOnly = 7,
    ReactivationRequest = 8,
    PDNReconnectiontothisAPNDisallowed = 9,
    AccessChangedfroNon3GPPto3GPP = 10,
    PDNConnectionInactivityTimerExpires = 11,
    PGWNotResponding = 12,
    NetworkFailure = 13,
    QoSParameterMismatch = 14,
    EPSto5GSMobility = 15,
    // Acceptance in a Response / Triggered message
    RequestAccepted = 16,
    RequestAcceptedPartially = 17,
    NewPDNTypeDuetoNetworkPerformance = 18,
    NewPDNTypeDuetoSingleAddressBearerOnly = 19,
    // Rejection in a Response / Triggered message
    ContextNotFound = 64,
    InvalidMessageFormat = 65,
    VersionNotSupportedByNextPeer = 66,
    InvalidLength = 67,
    ServiceNotSupported = 68,
    MandatoryIEIncorrect = 69,
    MandatoryIEMissing = 70,
    // ShallNotBeUsed = 71,
    SystemFailure = 72,
    NoResourcesAvailable = 73,
    SemanticErrorintheTFTOperation = 74,
    SyntacticErrorintheTFTOperation = 75,
    SemanticErrorsinthePacketFilters = 76,
    SyntacticErrorsinthePacketFilters = 77,
    MissingOrUnknownAPN = 78,
    // ShallNotBeUsed = 79,
    GREKeyNotFound = 80,
    RelocationFailure = 81,
    DeniedinRAT = 82,
    PreferredPDNTypeNotSupported = 83,
    AllDynamicAddressesAreOccupied = 84,
    UEContextWithoutTFTAlreadyActivated = 85,
    ProtocolTypeNotSupported = 86,
    UENotResponding = 87,
    UERefuses = 88,
    ServiceDenied = 89,
    UnableToPageUE = 90,
    NoMemoryAvailable = 91,
    UserAuthenticationFailed = 92,
    APNAccessDeniedNoSubscription = 93,
    RequestRejectedReasonNotSpecified = 94,
    PTMSISignatureMismatch = 95,
    IMSIorIMEINotKnown = 96,
    SemanticErrorintheTADOperation = 97,
    SyntacticErrorintheTADOperation = 98,
    // ShallNotBeUsed = 99,
    RemotePeerNotResponding = 100,
    CollisionWithNetworkInitiatedRequest = 101,
    UnableToPageUEDueToSuspension = 102,
    ConditionalIEMissing = 103,
    APNRestrictiontypeIncompatibleWithCurrentlyActivePDNConnection = 104,
    InvalidOverallLengthOfTheTriggeredResponseMessageAndAPiggybackedInitialMessage = 105,
    DataForwardingNotSupported = 106,
    InvalidReplyFromRemotePeer = 107,
    FallbackToGTPv1 = 108,
    InvalidPeer = 109,
    TemporarilyRejectedDueToHandoverOrTAUorRAUProcedureInProgress = 110,
    ModificationsNotLimitedToS1UBearers = 111,
    RequestRejectedForAPMIPv6Reason = 112,
    APNCongestion = 113,
    BearerHandlingNotSupported = 114,
    UEAlreadyReAttached = 115,
    MultiplePDNConnectionsForAGivenAPNNotAllowed = 116,
    TargetAccessRestrictedForTheSubscriber = 117,
    // ShallNotBeUsed = 118,
    MMESGSNRefusesDueToVPLMNPolicy = 119,
    CTPCEntityCongestion = 120,
    LateOverlappingRequest = 121,
    TimedOutRequest = 122,
    UEIsTemporarilyNotReachableDueToPowerSaving = 123,
    RelocationFailureDueToNASMessageRedirection = 124,
    UENotAuthorisedByOCSOrExternalAAAServer = 125,
    MultipleAccessesToAPDNConnectionNotAllowed = 126,
    RequestRejectedDueToUECapability = 127,
    S1UPathFailure = 128,
    _5GCNotAllowed = 129,

}

impl TryFrom<u8> for CauseCode
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            // 0 => Ok(CauseCode::ReservedShallNotBeSent),
            // Request / Initial Message
            1 => Ok(CauseCode::Reserved),
            2 => Ok(CauseCode::LocalDetach),
            3 => Ok(CauseCode::CompleteDetach),
            4 => Ok(CauseCode::RATChangedfrom3GPPtoNon3GPP),
            5 => Ok(CauseCode::ISRDeactivation),
            6 => Ok(CauseCode::ErrorIndicationReceivedfromRNCeNodeBS4SGSNMME),
            7 => Ok(CauseCode::IMSIDetachOnly),
            8 => Ok(CauseCode::ReactivationRequest),
            9 => Ok(CauseCode::PDNReconnectiontothisAPNDisallowed),
            10 => Ok(CauseCode::AccessChangedfroNon3GPPto3GPP),
            11 => Ok(CauseCode::PDNConnectionInactivityTimerExpires),
            12 => Ok(CauseCode::PGWNotResponding),
            13 => Ok(CauseCode::NetworkFailure),
            14 => Ok(CauseCode::QoSParameterMismatch),
            15 => Ok(CauseCode::EPSto5GSMobility),
            // Acceptance in a Response / Triggered message
            16 => Ok(CauseCode::RequestAccepted),
            17 => Ok(CauseCode::RequestAcceptedPartially),
            18 => Ok(CauseCode::NewPDNTypeDuetoNetworkPerformance),
            19 => Ok(CauseCode::NewPDNTypeDuetoSingleAddressBearerOnly),
            // Rejection in a Response / Triggered message
            64 => Ok(CauseCode::ContextNotFound),
            65 => Ok(CauseCode::InvalidMessageFormat),
            66 => Ok(CauseCode::VersionNotSupportedByNextPeer),
            67 => Ok(CauseCode::InvalidLength),
            68 => Ok(CauseCode::ServiceNotSupported),
            69 => Ok(CauseCode::MandatoryIEIncorrect),
            70 => Ok(CauseCode::MandatoryIEMissing),
            // 71 => Ok(CauseCode::ShallNotBeUsed),
            72 => Ok(CauseCode::SystemFailure),
            73 => Ok(CauseCode::NoResourcesAvailable),
            74 => Ok(CauseCode::SemanticErrorintheTFTOperation),
            75 => Ok(CauseCode::SyntacticErrorintheTFTOperation),
            76 => Ok(CauseCode::SemanticErrorsinthePacketFilters),
            77 => Ok(CauseCode::SyntacticErrorsinthePacketFilters),
            78 => Ok(CauseCode::MissingOrUnknownAPN),
            // 79 => Ok(CauseCode::ShallNotBeUsed),
            80 => Ok(CauseCode::GREKeyNotFound),
            81 => Ok(CauseCode::RelocationFailure),
            82 => Ok(CauseCode::DeniedinRAT),
            83 => Ok(CauseCode::PreferredPDNTypeNotSupported),
            84 => Ok(CauseCode::AllDynamicAddressesAreOccupied),
            85 => Ok(CauseCode::UEContextWithoutTFTAlreadyActivated),
            86 => Ok(CauseCode::ProtocolTypeNotSupported),
            87 => Ok(CauseCode::UENotResponding),
            88 => Ok(CauseCode::UERefuses),
            89 => Ok(CauseCode::ServiceDenied),
            90 => Ok(CauseCode::UnableToPageUE),
            91 => Ok(CauseCode::NoMemoryAvailable),
            92 => Ok(CauseCode::UserAuthenticationFailed),
            93 => Ok(CauseCode::APNAccessDeniedNoSubscription),
            94 => Ok(CauseCode::RequestRejectedReasonNotSpecified),
            95 => Ok(CauseCode::PTMSISignatureMismatch),
            96 => Ok(CauseCode::IMSIorIMEINotKnown),
            97 => Ok(CauseCode::SemanticErrorintheTADOperation),
            98 => Ok(CauseCode::SyntacticErrorintheTADOperation),
            // 99 => Ok(CauseCode::ShallNotBeUsed),
            100 => Ok(CauseCode::RemotePeerNotResponding),
            101 => Ok(CauseCode::CollisionWithNetworkInitiatedRequest),
            102 => Ok(CauseCode::UnableToPageUEDueToSuspension),
            103 => Ok(CauseCode::ConditionalIEMissing),
            104 => Ok(CauseCode::APNRestrictiontypeIncompatibleWithCurrentlyActivePDNConnection),
            105 => Ok(CauseCode::InvalidOverallLengthOfTheTriggeredResponseMessageAndAPiggybackedInitialMessage),
            106 => Ok(CauseCode::DataForwardingNotSupported),
            107 => Ok(CauseCode::InvalidReplyFromRemotePeer),
            108 => Ok(CauseCode::FallbackToGTPv1),
            109 => Ok(CauseCode::InvalidPeer),
            110 => Ok(CauseCode::TemporarilyRejectedDueToHandoverOrTAUorRAUProcedureInProgress),
            111 => Ok(CauseCode::ModificationsNotLimitedToS1UBearers),
            112 => Ok(CauseCode::RequestRejectedForAPMIPv6Reason),
            113 => Ok(CauseCode::APNCongestion),
            114 => Ok(CauseCode::BearerHandlingNotSupported),
            115 => Ok(CauseCode::UEAlreadyReAttached),
            116 => Ok(CauseCode::MultiplePDNConnectionsForAGivenAPNNotAllowed),
            117 => Ok(CauseCode::TargetAccessRestrictedForTheSubscriber),
            // 118 => Ok(CauseCode::ShallNotBeUsed),
            119 => Ok(CauseCode::MMESGSNRefusesDueToVPLMNPolicy),
            120 => Ok(CauseCode::CTPCEntityCongestion),
            121 => Ok(CauseCode::LateOverlappingRequest),
            122 => Ok(CauseCode::TimedOutRequest),
            123 => Ok(CauseCode::UEIsTemporarilyNotReachableDueToPowerSaving),
            124 => Ok(CauseCode::RelocationFailureDueToNASMessageRedirection),
            125 => Ok(CauseCode::UENotAuthorisedByOCSOrExternalAAAServer),
            126 => Ok(CauseCode::MultipleAccessesToAPDNConnectionNotAllowed),
            127 => Ok(CauseCode::RequestRejectedDueToUECapability),
            128 => Ok(CauseCode::S1UPathFailure),
            129 => Ok(CauseCode::_5GCNotAllowed),
            _ => Err(format!("Unsupported Cause Code ({})", value))
        }
    }
}

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (2)                                                   |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       | Cause Code                                                    |
        6       | Spare                                 | PCE   | BCE   | CS    |
        7       | Offending IE Type                                             |
        8       | Length Octet 1 (0)                                            |
        9       | Length Octet 2 (0)                                            |
        10      | Spare                         | Instance                      |
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub cause_code: CauseCode,
    pub cause_source: CauseSource,
    pub bearer_context_ie_error: bool,
    pub pdn_connection_ie_error: bool,
    pub offending_ie: Option<(InformationElementType, u8)>, // (IE Type, Instance)

}

impl InformationElement {
    pub fn new(
        cause_code: CauseCode,
        cause_source: CauseSource,
        bearer_context_ie_error: bool,
        pdn_connection_ie_error: bool,
        offending_ie: Option<(InformationElementType, u8)>,
        instance: u8
    ) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    cause_code,
                    cause_source,
                    bearer_context_ie_error,
                    pdn_connection_ie_error,
                    offending_ie,
                    instance,
                }
            )
        }
    }

    pub fn parseflags(buffer: &[u8]) -> Result<(bool, bool, CauseSource), String> {
        let cause_source = CauseSource::try_from(buffer[0] & 0x1)?;
       
        let bearer_context_ie_error = ((buffer[0] >> 1) & 0x1) == 1;

        let pdn_connection_ie_error = ((buffer[0] >> 2) & 0x1) == 1;

        Ok((pdn_connection_ie_error, bearer_context_ie_error, cause_source))
           
    }

    pub fn generateflags(&self) -> u8 {
        ((if self.pdn_connection_ie_error { 1 } else { 0 }) << 2) |
        ((if self.bearer_context_ie_error { 1 } else { 0 }) << 1) |
        (self.cause_source as u8 & 0x1)
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

        if let Ok(cause_code) = CauseCode::try_from(buffer[pos]) {
            pos = pos + 1;
            if let Ok((pdn_connection_ie_error, bearer_context_ie_error, cause_source)) = Self::parseflags(&buffer[pos..]) {
                pos = pos + 1;
                let offending_ie: Option<(InformationElementType, u8)>;

                match length {
                    2 => offending_ie = None,
                    6 => {
                        if let Ok(offending_ie_type) = InformationElementType::try_from(buffer[pos]) {
                            pos = pos + 1;

                            let _offending_ie_length = NetworkEndian::read_u16(&buffer[pos..pos+2]);
                            pos = pos + 2;
                            
                            let offending_ie_instance = buffer[pos] & 0xF;
                            // pos = pos + 1;

                            offending_ie = Some((offending_ie_type, offending_ie_instance));
                        }
                        else {
                            return None
                        }
                    }
                    _ => return None,
                }


                Some(
                    (
                        InformationElement {
                            cause_code,
                            cause_source,
                            bearer_context_ie_error,
                            pdn_connection_ie_error,
                            offending_ie,
                            instance,
                        },
                        (length + 4) as usize
                    )
                )
            }
            else { None }
        }
        else { None }
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::Cause
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

        let mut length = 4+2;

        if let Some(_) = self.offending_ie {
            length = length + 4;
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

        buffer[pos] = self.cause_code as u8;
        pos = pos + 1;

        buffer[pos] = self.generateflags();
        pos = pos + 1;

        if let Some((offending_ie_type, offending_ie_instance)) = self.offending_ie {
            buffer[pos] = offending_ie_type as u8;
            pos = pos + 1;

            NetworkEndian::write_u16(&mut buffer[pos..pos+2], 0);
            pos = pos + 2;

            buffer[pos] = offending_ie_instance & 0xF;
            pos = pos + 1;
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

        let ie = InformationElement::new(
            CauseCode::NoResourcesAvailable,
            CauseSource::LocalNode,
            false,
            true,
            Some((InformationElementType::BearerContext, 1)),
            0).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::Cause as u8,
            0, 6, // Length
            0, // Spare
            CauseCode::NoResourcesAvailable as u8,
            0b00000100,
            InformationElementType::BearerContext as u8,
            0, 0,
            0x01
        ]);

        let mut buffer = [0; MTU];

        let ie = InformationElement::new(
            CauseCode::RequestAccepted,
            CauseSource::LocalNode,
            false,
            false,
            None,
            0).unwrap();

        let pos = ie.generate(&mut buffer);

        assert_eq!(buffer[..pos], [InformationElementType::Cause as u8,
            0, 2, // Length
            0, // Spare
            CauseCode::RequestAccepted as u8,
            0b00000000,
        ]);
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(
            CauseCode::RequestAccepted,
            CauseSource::LocalNode,
            false,
            false,
            None,
            0).unwrap();

        assert_eq!(ie.length(), 2+4);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(
            CauseCode::RequestAccepted,
            CauseSource::LocalNode,
            false,
            false,
            None,
            0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::Cause as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::Cause as u8,
            0, 6, // Length
            0, // Spare
            CauseCode::NoResourcesAvailable as u8,
            0b00000100,
            InformationElementType::BearerContext as u8,
            0, 0,
            0x01
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            assert_eq!(ie.cause_code, CauseCode::NoResourcesAvailable);
            assert_eq!(ie.cause_source, CauseSource::LocalNode);
            assert_eq!(ie.pdn_connection_ie_error, true);
            assert_eq!(ie.bearer_context_ie_error, false);
            assert_eq!(ie.offending_ie, Some((InformationElementType::BearerContext, 1)))
            
        }
        else {
            assert!(false);
        }
    }
}