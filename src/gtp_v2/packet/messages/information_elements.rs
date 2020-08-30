pub mod recovery;
pub mod rat_type;
pub mod f_teid;
pub mod apn;
pub mod bearer_context;
pub mod ebi;
pub mod bearer_qos;
pub mod imsi;
pub mod pdn_type;
pub mod pdn_address_allocation;
pub mod msisdn;
pub mod user_location_information;
pub mod mei;
pub mod serving_network;
pub mod selection_mode;
pub mod apn_restriction;
pub mod ambr;
pub mod ue_time_zone;
pub mod charging_characteristics;
pub mod cause;

use byteorder::{ByteOrder, NetworkEndian};

use std::convert::TryFrom;
use std::convert::TryInto;

use crate::field::*;
pub const LENGTH: Field = 1..3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InformationElementType
{
    Recovery = 3,
    RATType = 82,
    FTEID = 87,
    APN = 71,
    BearerContext = 93,
    EBI = 73,
    BearerQoS = 80,
    IMSI = 1,
    PDNType = 99,
    PDNAddressAllocation = 79,
    MSISDN = 76,
    UserLocationInformation = 86,
    MEI = 75,
    ServingNetwork = 83,
    SelectionMode = 128,
    APNRestriction = 127,
    AMBR = 72,
    UETimeZone = 114,
    ChargingCharacteristics = 95,
    Cause = 2,
}

impl TryFrom<u8> for InformationElementType
{
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(InformationElementType::Recovery),
            82 => Ok(InformationElementType::RATType),
            87 => Ok(InformationElementType::FTEID),
            71 => Ok(InformationElementType::APN),
            93 => Ok(InformationElementType::BearerContext),
            73 => Ok(InformationElementType::EBI),
            80 => Ok(InformationElementType::BearerQoS),
            1 => Ok(InformationElementType::IMSI),
            99 => Ok(InformationElementType::PDNType),
            79 => Ok(InformationElementType::PDNAddressAllocation),
            76 => Ok(InformationElementType::MSISDN),
            86 => Ok(InformationElementType::UserLocationInformation),
            75 => Ok(InformationElementType::UserLocationInformation),
            83 => Ok(InformationElementType::ServingNetwork),
            128 => Ok(InformationElementType::SelectionMode),
            127 => Ok(InformationElementType::APNRestriction),
            72 => Ok(InformationElementType::AMBR),
            114 => Ok(InformationElementType::UETimeZone),
            95 => Ok(InformationElementType::ChargingCharacteristics),
            2 => Ok(InformationElementType::Cause),
            _ => Err(format!("Unsupported IE type ({})", value).to_string())
        }
    }
}

pub trait InformationElementTraits {
    fn information_element_type(&self) -> InformationElementType;
    fn length(&self) -> u16;
    fn generate(&self, buffer: &mut[u8]) -> usize;
    fn instance(&self) -> u8;
    fn set_instance(&mut self, instance: u8) -> Result<u8, String>;
}

pub enum InformationElement
{
    Recovery(recovery::InformationElement),
    RATType(rat_type::InformationElement),
    FTEID(f_teid::InformationElement),
    APN(apn::InformationElement),
    BearerContext(bearer_context::InformationElement),
    EBI(ebi::InformationElement),
    BearerQoS(bearer_qos::InformationElement),
    IMSI(imsi::InformationElement),
    PDNType(pdn_type::InformationElement),
    PDNAddressAllocation(pdn_address_allocation::InformationElement),
    MSISDN(msisdn::InformationElement),
    ULI(user_location_information::InformationElement),
    MEI(mei::InformationElement),
    ServingNetwork(serving_network::InformationElement),
    SelectionMode(selection_mode::InformationElement),
    APNRestriction(apn_restriction::InformationElement),
    AMBR(ambr::InformationElement),
    UETimeZone(ue_time_zone::InformationElement),
    ChargingCharacteristics(charging_characteristics::InformationElement),
    Cause(cause::InformationElement),
}

impl InformationElement {
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        // First we parse the first byte to determine the type of the IE.
        // Then we pass off the parsing to the specific IE implementation

        let ie_type = buffer[0];

        if let Ok(ie_type) = ie_type.try_into() {
            match ie_type {
                InformationElementType::Recovery => {
                    if let Some((ie, pos)) = recovery::InformationElement::parse(buffer) {
                        Some((InformationElement::Recovery(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::ChargingCharacteristics => {
                    if let Some((ie, pos)) = charging_characteristics::InformationElement::parse(buffer) {
                        Some((InformationElement::ChargingCharacteristics(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::Cause => {
                    if let Some((ie, pos)) = cause::InformationElement::parse(buffer) {
                        Some((InformationElement::Cause(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::UETimeZone => {
                    if let Some((ie, pos)) = ue_time_zone::InformationElement::parse(buffer) {
                        Some((InformationElement::UETimeZone(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::AMBR => {
                    if let Some((ie, pos)) = ambr::InformationElement::parse(buffer) {
                        Some((InformationElement::AMBR(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::APNRestriction => {
                    if let Some((ie, pos)) = apn_restriction::InformationElement::parse(buffer) {
                        Some((InformationElement::APNRestriction(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::SelectionMode => {
                    if let Some((ie, pos)) = selection_mode::InformationElement::parse(buffer) {
                        Some((InformationElement::SelectionMode(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::ServingNetwork => {
                    if let Some((ie, pos)) = serving_network::InformationElement::parse(buffer) {
                        Some((InformationElement::ServingNetwork(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::MEI => {
                    if let Some((ie, pos)) = mei::InformationElement::parse(buffer) {
                        Some((InformationElement::MEI(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::UserLocationInformation => {
                    if let Some((ie, pos)) = user_location_information::InformationElement::parse(buffer) {
                        Some((InformationElement::ULI(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::PDNAddressAllocation => {
                    if let Some((ie, pos)) = pdn_address_allocation::InformationElement::parse(buffer) {
                        Some((InformationElement::PDNAddressAllocation(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::PDNType => {
                    if let Some((ie, pos)) = pdn_type::InformationElement::parse(buffer) {
                        Some((InformationElement::PDNType(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::IMSI => {
                    if let Some((ie, pos)) = imsi::InformationElement::parse(buffer) {
                        Some((InformationElement::IMSI(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::MSISDN => {
                    if let Some((ie, pos)) = msisdn::InformationElement::parse(buffer) {
                        Some((InformationElement::MSISDN(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::RATType => {
                    if let Some((ie, pos)) = rat_type::InformationElement::parse(buffer) {
                        Some((InformationElement::RATType(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::APN => {
                    if let Some((ie, pos)) = apn::InformationElement::parse(buffer) {
                        Some((InformationElement::APN(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::FTEID => {
                    if let Some((ie, pos)) = f_teid::InformationElement::parse(buffer) {
                        Some((InformationElement::FTEID(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::BearerContext => {
                    if let Some((ie, pos)) = bearer_context::InformationElement::parse(buffer) {
                        Some((InformationElement::BearerContext(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::EBI => {
                    if let Some((ie, pos)) = ebi::InformationElement::parse(buffer) {
                        Some((InformationElement::EBI(ie), pos))
                    }
                    else {
                        None
                    }
                },
                InformationElementType::BearerQoS => {
                    if let Some((ie, pos)) = bearer_qos::InformationElement::parse(buffer) {
                        Some((InformationElement::BearerQoS(ie), pos))
                    }
                    else {
                        None
                    }
                },
                // _ => {
                //     None
                // }
            }
        }
        else {
            None
        }
    }

    pub fn skip_parsing(buffer: &[u8]) -> usize {
        // This function will return the number of bytes to advance the buffer to skip over parsing this IE.
        // This is useful if we encounter an IE that we can't parse but want to continue to try to parse other elements.

        // Read length of IE
        let length = NetworkEndian::read_u16(&buffer[LENGTH]);
        
        // Add 4 for IE header
        (length + 4) as usize
    }
}

impl InformationElementTraits for InformationElement
{
    fn length(&self) -> u16 {
        match self {
            InformationElement::Recovery(ie) => ie.length(),
            InformationElement::RATType(ie) => ie.length(),
            InformationElement::APN(ie) => ie.length(),
            InformationElement::FTEID(ie) => ie.length(),
            InformationElement::BearerContext(ie) => ie.length(),
            InformationElement::EBI(ie) => ie.length(),
            InformationElement::BearerQoS(ie) => ie.length(),
            InformationElement::IMSI(ie) => ie.length(),
            InformationElement::PDNType(ie) => ie.length(),
            InformationElement::PDNAddressAllocation(ie) => ie.length(),
            InformationElement::MSISDN(ie) => ie.length(),
            InformationElement::ULI(ie) => ie.length(),
            InformationElement::MEI(ie) => ie.length(),
            InformationElement::ServingNetwork(ie) => ie.length(),
            InformationElement::SelectionMode(ie) => ie.length(),
            InformationElement::APNRestriction(ie) => ie.length(),
            InformationElement::AMBR(ie) => ie.length(),
            InformationElement::UETimeZone(ie) => ie.length(),
            InformationElement::ChargingCharacteristics(ie) => ie.length(),
            InformationElement::Cause(ie) => ie.length(),
        }
    }

    fn instance(&self) -> u8 {
        match self {
            InformationElement::Recovery(ie) => ie.instance(),
            InformationElement::RATType(ie) => ie.instance(),
            InformationElement::APN(ie) => ie.instance(),
            InformationElement::FTEID(ie) => ie.instance(),
            InformationElement::BearerContext(ie) => ie.instance(),
            InformationElement::EBI(ie) => ie.instance(),
            InformationElement::BearerQoS(ie) => ie.instance(),
            InformationElement::IMSI(ie) => ie.instance(),
            InformationElement::PDNType(ie) => ie.instance(),
            InformationElement::PDNAddressAllocation(ie) => ie.instance(),
            InformationElement::MSISDN(ie) => ie.instance(),
            InformationElement::ULI(ie) => ie.instance(),
            InformationElement::MEI(ie) => ie.instance(),
            InformationElement::ServingNetwork(ie) => ie.instance(),
            InformationElement::SelectionMode(ie) => ie.instance(),
            InformationElement::APNRestriction(ie) => ie.instance(),
            InformationElement::AMBR(ie) => ie.instance(),
            InformationElement::UETimeZone(ie) => ie.instance(),
            InformationElement::ChargingCharacteristics(ie) => ie.instance(),
            InformationElement::Cause(ie) => ie.instance(),
        }
    }

    fn set_instance(&mut self, instance: u8) -> Result<u8, String> {
        match self {
            InformationElement::Recovery(ie) => ie.set_instance(instance),
            InformationElement::RATType(ie) => ie.set_instance(instance),
            InformationElement::APN(ie) => ie.set_instance(instance),
            InformationElement::FTEID(ie) => ie.set_instance(instance),
            InformationElement::BearerContext(ie) => ie.set_instance(instance),
            InformationElement::EBI(ie) => ie.set_instance(instance),
            InformationElement::BearerQoS(ie) => ie.set_instance(instance),
            InformationElement::IMSI(ie) => ie.set_instance(instance),
            InformationElement::PDNType(ie) => ie.set_instance(instance),
            InformationElement::PDNAddressAllocation(ie) => ie.set_instance(instance),
            InformationElement::MSISDN(ie) => ie.set_instance(instance),
            InformationElement::ULI(ie) => ie.set_instance(instance),
            InformationElement::MEI(ie) => ie.set_instance(instance),
            InformationElement::ServingNetwork(ie) => ie.set_instance(instance),
            InformationElement::SelectionMode(ie) => ie.set_instance(instance),
            InformationElement::APNRestriction(ie) => ie.set_instance(instance),
            InformationElement::AMBR(ie) => ie.set_instance(instance),
            InformationElement::UETimeZone(ie) => ie.set_instance(instance),
            InformationElement::ChargingCharacteristics(ie) => ie.set_instance(instance),
            InformationElement::Cause(ie) => ie.set_instance(instance),
        }
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        match self {
            InformationElement::Recovery(ie) => ie.generate(buffer),
            InformationElement::RATType(ie) => ie.generate(buffer),
            InformationElement::APN(ie) => ie.generate(buffer),
            InformationElement::FTEID(ie) => ie.generate(buffer),
            InformationElement::BearerContext(ie) => ie.generate(buffer),
            InformationElement::EBI(ie) => ie.generate(buffer),
            InformationElement::BearerQoS(ie) => ie.generate(buffer),
            InformationElement::IMSI(ie) => ie.generate(buffer),
            InformationElement::PDNType(ie) => ie.generate(buffer),
            InformationElement::PDNAddressAllocation(ie) => ie.generate(buffer),
            InformationElement::MSISDN(ie) => ie.generate(buffer),
            InformationElement::ULI(ie) => ie.generate(buffer),
            InformationElement::MEI(ie) => ie.generate(buffer),
            InformationElement::ServingNetwork(ie) => ie.generate(buffer),
            InformationElement::SelectionMode(ie) => ie.generate(buffer),
            InformationElement::APNRestriction(ie) => ie.generate(buffer),
            InformationElement::AMBR(ie) => ie.generate(buffer),
            InformationElement::UETimeZone(ie) => ie.generate(buffer),
            InformationElement::ChargingCharacteristics(ie) => ie.generate(buffer),
            InformationElement::Cause(ie) => ie.generate(buffer),
        }
    }

    fn information_element_type(&self) -> InformationElementType {
        match self {
            InformationElement::Recovery(ie) => ie.information_element_type(),
            InformationElement::RATType(ie) => ie.information_element_type(),
            InformationElement::APN(ie) => ie.information_element_type(),
            InformationElement::FTEID(ie) => ie.information_element_type(),
            InformationElement::BearerContext(ie) => ie.information_element_type(),
            InformationElement::EBI(ie) => ie.information_element_type(),
            InformationElement::BearerQoS(ie) => ie.information_element_type(),
            InformationElement::IMSI(ie) => ie.information_element_type(),
            InformationElement::PDNType(ie) => ie.information_element_type(),
            InformationElement::PDNAddressAllocation(ie) => ie.information_element_type(),
            InformationElement::MSISDN(ie) => ie.information_element_type(),
            InformationElement::ULI(ie) => ie.information_element_type(),
            InformationElement::MEI(ie) => ie.information_element_type(),
            InformationElement::ServingNetwork(ie) => ie.information_element_type(),
            InformationElement::SelectionMode(ie) => ie.information_element_type(),
            InformationElement::APNRestriction(ie) => ie.information_element_type(),
            InformationElement::AMBR(ie) => ie.information_element_type(),
            InformationElement::UETimeZone(ie) => ie.information_element_type(),
            InformationElement::ChargingCharacteristics(ie) => ie.information_element_type(),
            InformationElement::Cause(ie) => ie.information_element_type(),
        }
    }
}