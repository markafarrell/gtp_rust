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
    fn information_element_type(&self) -> InformationElementType;
    fn length(&self) -> u16;
    fn generate(&self, buffer: &mut[u8]) -> usize;
    fn parse(&mut self, buffer: &[u8]);
}