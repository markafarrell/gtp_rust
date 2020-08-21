pub mod imsi;
pub mod teid_data_i;
pub mod nsapi;
pub mod gsn_address;
pub mod qos_profile;

#[derive(Copy, Clone, Debug)]
pub enum InformationElementType
{
    Imsi = 2,
    TeidDataI = 16,
    Nsapi = 20,
    GsnAddress = 133,
    QoSProfile = 135,
}

pub trait InformationElementTraits {
    fn length(&self) -> u16;
    fn generate(&self, buffer: &mut[u8]) -> usize;
    fn information_element_type(&self) -> InformationElementType;
}

pub enum InformationElement
{
    Imsi(imsi::InformationElement),
    TeidDataI(teid_data_i::InformationElement),
    Nsapi(nsapi::InformationElement),
    GsnAddress(gsn_address::InformationElement),
    QoSProfile(qos_profile::InformationElement),
}

impl InformationElementTraits for InformationElement
{
    fn length(&self) -> u16 {
        match self {
            InformationElement::Imsi(ie) => ie.length(),
            InformationElement::TeidDataI(ie) => ie.length(),
            InformationElement::Nsapi(ie) => ie.length(),
            InformationElement::GsnAddress(ie) => ie.length(),
            InformationElement::QoSProfile(ie) => ie.length(),
        }
    }

    fn generate(&self, buffer: &mut[u8]) -> usize {
        match self {
            InformationElement::Imsi(ie) => ie.generate(buffer),
            InformationElement::TeidDataI(ie) => ie.generate(buffer),
            InformationElement::Nsapi(ie) => ie.generate(buffer),
            InformationElement::GsnAddress(ie) => ie.generate(buffer),
            InformationElement::QoSProfile(ie) => ie.generate(buffer),
        }
    }

    fn information_element_type(&self) -> InformationElementType {
        match self {
            InformationElement::Imsi(ie) => ie.information_element_type(),
            InformationElement::TeidDataI(ie) => ie.information_element_type(),
            InformationElement::Nsapi(ie) => ie.information_element_type(),
            InformationElement::GsnAddress(ie) => ie.information_element_type(),
            InformationElement::QoSProfile(ie) => ie.information_element_type(),
        }
    }
}