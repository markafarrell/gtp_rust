use byteorder::{ByteOrder, NetworkEndian};

use super::{InformationElementTraits, InformationElementType, LENGTH};

use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PLMN {
    pub mcc: [u8; 3],
    pub mnc: [u8; 3],
}

impl PLMN {
    pub fn new(mcc: [u8; 3], mnc: [u8; 3])-> Self
    {
        Self {
            mcc,
            mnc,
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mcc = [
            (buffer[0] & 0xF), ((buffer[0] >> 4) & 0xF), (buffer[1] & 0xF), 
        ];

        let mnc = [
            (buffer[2] & 0xF), ((buffer[2] >> 4) & 0xF), ((buffer[1] >> 4) & 0xF), 
        ];

        Some(
            (
                Self {
                    mcc,
                    mnc
                },
                3,
            )
        )

    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = 0;

        buffer[pos] = (self.mcc[1] << 4) | (self.mcc[0] & 0xF);
        pos = pos + 1;

        buffer[pos] = (self.mnc[2] << 4) | (self.mcc[2] & 0xF);
        pos = pos + 1;

        buffer[pos] = (self.mnc[1] << 4) | (self.mnc[0] & 0xF);
        pos = pos + 1;

        pos
    }
    pub fn length(&self) -> u16 {3}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CGI {
    pub plmn: PLMN,
    pub lac: u16,
    pub ci: u16,
}

impl CGI {
    pub fn new(plmn: PLMN, lac: u16, ci: u16)-> Self
    {
        Self {
            plmn,
            lac,
            ci,
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        if let Some((plmn, plmn_pos)) = PLMN::parse(buffer) {
            pos = pos + plmn_pos;
            let lac = NetworkEndian::read_u16(&buffer[pos..pos+2]);
            pos = pos + 2;

            let ci = NetworkEndian::read_u16(&buffer[pos..pos+2]);
            pos = pos + 2;

            Some(
                (
                    Self {
                        plmn,
                        lac,
                        ci,
                    }, 
                    pos
                )
            )
        }
        else {
            None
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = self.plmn.generate(buffer);

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.lac);
        pos = pos + 2;

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.ci);
        pos = pos + 2;

        pos
    }
    pub fn length(&self) -> u16 {self.plmn.length() + 4}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SAI {
    pub plmn: PLMN,
    pub lac: u16,
    pub sac: u16,
}

impl SAI {
    pub fn new(plmn: PLMN, lac: u16, sac: u16)-> Self
    {
        Self {
            plmn,
            sac,
            lac,
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        if let Some((plmn, plmn_pos)) = PLMN::parse(buffer) {
            pos = pos + plmn_pos;
            let lac = NetworkEndian::read_u16(&buffer[pos..pos+2]);
            pos = pos + 2;

            let sac = NetworkEndian::read_u16(&buffer[pos..pos+2]);
            pos = pos + 2;

            Some(
                (
                    Self {
                        plmn,
                        lac,
                        sac,
                    }, 
                    pos
                )
            )
        }
        else {
            None
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = self.plmn.generate(buffer);

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.lac);
        pos = pos + 2;

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.sac);
        pos = pos + 2;

        pos
    }
    pub fn length(&self) -> u16 {self.plmn.length() + 4}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RAI {
    pub plmn: PLMN,
    pub lac: u16,
    pub rac: u16,
}

impl RAI {
    pub fn new(plmn: PLMN, lac: u16, rac: u16)-> Self
    {
        Self {
            plmn,
            rac,
            lac,
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        if let Some((plmn, plmn_pos)) = PLMN::parse(buffer) {
            pos = pos + plmn_pos;
            let lac = NetworkEndian::read_u16(&buffer[pos..pos+2]);
            pos = pos + 2;

            let rac = NetworkEndian::read_u16(&buffer[pos..pos+2]);
            pos = pos + 2;

            Some(
                (
                    Self {
                        plmn,
                        lac,
                        rac,
                    }, 
                    pos
                )
            )
        }
        else {
            None
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = self.plmn.generate(buffer);

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.lac);
        pos = pos + 2;

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.rac);
        pos = pos + 2;

        pos
    }
    pub fn length(&self) -> u16 {self.plmn.length() + 4}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TAI {
    pub plmn: PLMN,
    pub tac: u16,
}

impl TAI {
    pub fn new(plmn: PLMN, tac: u16)-> Self
    {
        Self {
            plmn,
            tac,
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        if let Some((plmn, plmn_pos)) = PLMN::parse(buffer) {
            pos = pos + plmn_pos;

            let tac = NetworkEndian::read_u16(&buffer[pos..pos+2]);
            pos = pos + 2;

            Some(
                (
                    Self {
                        plmn,
                        tac,
                    }, 
                    pos
                )
            )
        }
        else {
            None
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = self.plmn.generate(buffer);

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.tac);
        pos = pos + 2;

        pos
    }
    pub fn length(&self) -> u16 {self.plmn.length() + 2}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ECGI {
    pub plmn: PLMN,
    eci: u32
}

impl ECGI {
    pub fn new(plmn: PLMN, eci: u32)-> Result<Self,String>
    {
        if eci > 0xFFFFFFF {
            Err("ECI must be <= 0xFFFFFFF".to_string())
        }
        else
        {
            Ok(
                Self {
                    plmn,
                    eci
                }
            )
        }
    }

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        if let Some((plmn, plmn_pos)) = PLMN::parse(buffer) {
            pos = pos + plmn_pos;
            let eci = NetworkEndian::read_u32(&buffer[pos..pos+4]) & 0xF_FF_FF_FF;
            pos = pos + 4;

            Some(
                (
                    Self {
                        plmn,
                        eci: eci,
                    }, 
                    pos
                )
            )
        }
        else {
            None
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = self.plmn.generate(buffer);

        NetworkEndian::write_u32(&mut buffer[pos..pos+4], self.eci);
        pos = pos + 4;

        pos
    }
    pub fn length(&self) -> u16 {self.plmn.length() + 4}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LAI {
    pub plmn: PLMN,
    pub lac: u16,
}

impl LAI {
    pub fn new(plmn: PLMN, lac: u16)-> Self
    {
        Self {
            plmn,
            lac,
        }
    }
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        if let Some((plmn, plmn_pos)) = PLMN::parse(buffer) {
            pos = pos + plmn_pos;
            let lac = NetworkEndian::read_u16(&buffer[pos..pos+2]);
            pos = pos + 2;

            Some(
                (
                    Self {
                        plmn,
                        lac,
                    }, 
                    pos
                )
            )
        }
        else {
            None
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = self.plmn.generate(buffer);

        NetworkEndian::write_u16(&mut buffer[pos..pos+2], self.lac);
        pos = pos + 2;

        pos
    }
    pub fn length(&self) -> u16 {self.plmn.length() + 2}

}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MeNBID {
    pub plmn: PLMN,
    menbid: u32
}

impl MeNBID {
    pub fn new(plmn: PLMN, menbid: u32)-> Result<Self,String>
    {
        if menbid > 0xFFFFF {
            Err("Macro eNodeB ID must be <= 0xFFFFF".to_string())
        }
        else
        {
            Ok(
                Self {
                    plmn,
                    menbid
                }
            )
        }
    }

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        if let Some((plmn, plmn_pos)) = PLMN::parse(buffer) {
            pos = pos + plmn_pos;
            let menbid = NetworkEndian::read_uint(&buffer[pos..pos+3], 3);
            pos = pos + 3;

            Some(
                (
                    Self {
                        plmn,
                        menbid: menbid.try_into().unwrap(),
                    }, 
                    pos
                )
            )
        }
        else {
            None
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = self.plmn.generate(buffer);

        NetworkEndian::write_uint(&mut buffer[pos..pos+3], self.menbid.into(), 3);
        pos = pos + 3;

        pos
    }
    pub fn length(&self) -> u16 {self.plmn.length() + 3}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EMeNBID {
    pub plmn: PLMN,
    menbid: u32
}

impl EMeNBID {
    pub fn new(plmn: PLMN, menbid: u32)-> Result<Self,String>
    {
        if menbid > 0x1FFFFF {
            Err("Extended Macro eNodeB ID must be <= 0x1FFFFF".to_string())
        }
        else
        {
            Ok(
                Self {
                    plmn,
                    menbid
                }
            )
        }
    }

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;

        if let Some((plmn, plmn_pos)) = PLMN::parse(buffer) {
            pos = pos + plmn_pos;

            //Get SMeNB Flag
            let smenb = buffer[pos] >> 7;

            let mut menbid: u64;

            if smenb == 1 {
                // We have a short Macro eNB ID (18 bits)
                menbid = NetworkEndian::read_uint(&buffer[pos..pos+3], 3);
                menbid = 0x3_FF_FF & menbid; // Blank out the top 6 bits
            }
            else 
            {
                // We have a Long Macro eNB ID (22 bits)
                menbid = NetworkEndian::read_uint(&buffer[pos..pos+3], 3);
                menbid = 0x1F_FF_FF & menbid; // Blank out the top 3 bits
            }

            pos = pos + 3;

            Some(
                (
                    Self {
                        plmn,
                        menbid: menbid.try_into().unwrap(),
                    }, 
                    pos
                )
            )
        }
        else {
            None
        }
    }
    pub fn generate(&self, buffer: &mut[u8]) -> usize {
        let mut pos = self.plmn.generate(buffer);

        if self.menbid < 0x3FFFF {
            // We have a short Macro eNB ID (18 bits)
            NetworkEndian::write_uint(&mut buffer[pos..pos+3], (self.menbid | (0x1 << 23)).into(), 3);
        }
        else {
            // We have a Long Macro eNB ID (22 bits)
            NetworkEndian::write_uint(&mut buffer[pos..pos+3], self.menbid.into(), 3);
        }
        pos = pos + 3;

        pos
    }
    pub fn length(&self) -> u16 {self.plmn.length() + 3}
}

pub struct InformationElement {

        /*
                                        Bits
                |---------------------------------------------------------------| 
        Octets  |   8   |   7   |   6   |   5   |   4   |   3   |   2   |   1   |
                |---------------------------------------------------------------|
        1       | IE Type (86)                                                  |
        2       | Length Octet 1                                                |
        3       | Length Octet 2                                                |
        4       | Spare                         | Instance                      |
        5       |EMeNBID| MeNBID| LAI   | ECGI  | TAI   | RAI   | SAI   | CGI   |
                | MCC Digit 2                   | MCC Digit 1                   | * If CGI flag set
                | MNC Digit 3                   | MCC Digit 3                   | * If CGI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If CGI flag set
                | LAC Octet 1                                                   | * If CGI flag set
                | LAC Octet 2                                                   | * If CGI flag set
                | Cell Identity Octet 1                                         | * If CGI flag set
                | Cell Identity Octet 2                                         | * If CGI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If SAI flag set
                | MNC Digit 3                   | MCC Digit 3                   | * If SAI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If SAI flag set
                | LAC Octet 1                                                   | * If SAI flag set
                | LAC Octet 2                                                   | * If SAI flag set
                | Service Area Code Octet 1                                     | * If SAI flag set
                | Service Area Code Octet 2                                     | * If SAI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If RAI flag set
                | MNC Digit 3                   | MCC Digit 3                   | * If RAI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If RAI flag set
                | LAC Octet 1                                                   | * If RAI flag set
                | LAC Octet 2                                                   | * If RAI flag set
                | Routing Area Code Octet 1                                     | * If RAI flag set
                | Routing Area Code Octet 2                                     | * If RAI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If TAI flag set
                | MNC Digit 3                   | MCC Digit 3                   | * If TAI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If TAI flag set
                | TAC Octet 1                                                   | * If TAI flag set
                | TAC Octet 2                                                   | * If TAI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If ECGI flag set
                | MNC Digit 3                   | MCC Digit 3                   | * If ECGI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If ECGI flag set
                | Spare                         | ECGI Octet 0.5                | * If ECGI flag set
                | ECGI Octet 1.5                                                | * If ECGI flag set
                | ECGI Octet 2.5                                                | * If ECGI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If LAI flag set
                | MNC Digit 3                   | MCC Digit 3                   | * If LAI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If LAI flag set
                | LAC Octet 1                                                   | * If LAI flag set
                | LAC Octet 2                                                   | * If LAI flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If MeNBID flag set
                | MNC Digit 3                   | MCC Digit 3                   | * If MeNBID flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If MeNBID flag set
                | Spare                         | MeNBID Octet 0.5              | * If MeNBID flag set
                | MeNBID Octet 1.5                                              | * If MeNBID flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If EMeNBID flag set
                | MNC Digit 3                   | MCC Digit 3                   | * If EMeNBID flag set
                | MCC Digit 2                   | MCC Digit 1                   | * If EMeNBID flag set
                | SMeNB | Spare         | MeNBID Bits 1-5                       | * If EMeNBID flag set
                | MeNBID Bits 6-13                                              | * If EMeNBID flag set
                | MeNBID Bits 14-21                                             | * If EMeNBID flag set
                |---------------------------------------------------------------|
    */

    instance: u8,
    pub cgi: Option<CGI>,
    pub sai: Option<SAI>,
    pub rai: Option<RAI>,
    pub tai: Option<TAI>,
    pub ecgi: Option<ECGI>,
    pub lai: Option<LAI>,
    pub menbid: Option<MeNBID>,
    pub emenbid: Option<EMeNBID>,
}

impl InformationElement {
    pub fn new(
        cgi: Option<CGI>,
        sai: Option<SAI>,
        rai: Option<RAI>,
        tai: Option<TAI>,
        ecgi: Option<ECGI>,
        lai: Option<LAI>,
        menbid: Option<MeNBID>,
        emenbid: Option<EMeNBID>,
        instance: u8
    ) -> Result<Self, String> {
        if instance > 0xF {
            Err(format!("Instance is > 0xF {}", instance))
        }
        else {
            Ok(
                InformationElement {
                    cgi,
                    sai,
                    rai,
                    tai,
                    ecgi,
                    lai,
                    menbid,
                    emenbid,
                    instance,
                }
            )
        }
    }

    pub fn parseflags(buffer: &[u8]) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
        (
            (buffer[0] >> 7) & 0x1, // EMeNBID
            (buffer[0] >> 6) & 0x1, // MeNBID
            (buffer[0] >> 5) & 0x1, // LAI
            (buffer[0] >> 4) & 0x1, // ECGI
            (buffer[0] >> 3) & 0x1, // TAI
            (buffer[0] >> 2) & 0x1, // RAI
            (buffer[0] >> 1) & 0x1, // SAI
            buffer[0] & 0x1, // CGI
        )
    }

    pub fn generateflags(&self) -> u8 {
        ((self.emenbid.is_some() as u8) << 7) | // EMeNBID
        ((self.menbid.is_some() as u8) << 6) | // MeNBID
        ((self.lai.is_some() as u8) << 5) | // LAI
        ((self.ecgi.is_some() as u8) << 4) | // ECGI
        ((self.tai.is_some() as u8) << 3) | // TAI
        ((self.rai.is_some() as u8) << 2) | // RAI
        ((self.sai.is_some() as u8) << 1) | // SAI
        (self.cgi.is_some() as u8) // CGI
    }

    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        let mut pos = 0;
        
        // Read the type
        let _ie_type = buffer[pos];
        pos = pos + 1;

        // Read the length
        let _length = NetworkEndian::read_u16(&buffer[LENGTH]);
        pos = pos + 2;

        //Spare and instance
        let instance = buffer[pos] & 0xF;
        pos = pos + 1;

        let (
            emenbid_flag, 
            menbid_flag, 
            lai_flag, 
            ecgi_flag, 
            tai_flag, 
            rai_flag, 
            sai_flag, 
            cgi_flag,
        ) = Self::parseflags(&buffer[pos..pos+1]);
        pos = pos + 1;

        let mut cgi: Option<CGI> = None;
        let mut sai: Option<SAI> = None;
        let mut rai: Option<RAI> = None;
        let mut tai: Option<TAI> = None;
        let mut ecgi: Option<ECGI> = None;
        let mut lai: Option<LAI> = None;
        let mut menbid: Option<MeNBID> = None;
        let mut emenbid: Option<EMeNBID> = None;

        if cgi_flag == 1 {
            if let Some((li, li_pos)) = CGI::parse(&buffer[pos..]) {
                cgi = Some(li);
                pos = pos + li_pos
            }
            else {
                return None
            }
        }

        if sai_flag == 1 {
            if let Some((li, li_pos)) = SAI::parse(&buffer[pos..]) {
                sai = Some(li);
                pos = pos + li_pos
            }
            else {
                return None
            }
        }

        if rai_flag == 1 {
            if let Some((li, li_pos)) = RAI::parse(&buffer[pos..]) {
                rai = Some(li);
                pos = pos + li_pos
            }
            else {
                return None
            }
        }

        if tai_flag == 1 {
            if let Some((li, li_pos)) = TAI::parse(&buffer[pos..]) {
                tai = Some(li);
                pos = pos + li_pos
            }
            else {
                return None
            }
        }

        if ecgi_flag == 1 {
            if let Some((li, li_pos)) = ECGI::parse(&buffer[pos..]) {
                ecgi = Some(li);
                pos = pos + li_pos
            }
            else {
                return None
            }
        }

        if lai_flag == 1 {
            if let Some((li, li_pos)) = LAI::parse(&buffer[pos..]) {
                lai = Some(li);
                pos = pos + li_pos
            }
            else {
                return None
            }
        }

        if menbid_flag == 1 {
            if let Some((li, li_pos)) = MeNBID::parse(&buffer[pos..]) {
                menbid = Some(li);
                pos = pos + li_pos
            }
            else {
                return None
            }
        }

        if emenbid_flag == 1 {
            if let Some((li, li_pos)) = EMeNBID::parse(&buffer[pos..]) {
                emenbid = Some(li);
                pos = pos + li_pos
            }
            else {
                return None
            }
        }

        Some(
            (
            InformationElement {
                cgi,
                sai,
                rai,
                tai,
                ecgi,
                lai,
                menbid,
                emenbid,
                instance,
            },
            pos)
        )
    }
}

impl InformationElementTraits for InformationElement {
    fn information_element_type(&self) -> InformationElementType {
        InformationElementType::UserLocationInformation
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

        let mut length = 4;

        length = length + 1; // Flags

        if let Some(li) = &self.cgi {
            length = length + li.length();
        }
        if let Some(li) = &self.sai {
            length = length + li.length();
        }
        if let Some(li) = &self.rai {
            length = length + li.length();
        }
        if let Some(li) = &self.tai {
            length = length + li.length();
        }
        if let Some(li) = &self.ecgi {
            length = length + li.length();
        }
        if let Some(li) = &self.lai {
            length = length + li.length();
        }
        if let Some(li) = &self.menbid {
            length = length + li.length();
        }
        if let Some(li) = &self.emenbid {
            length = length + li.length();
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

        buffer[pos] = self.generateflags();
        pos = pos + 1;

        if let Some(li) = &self.cgi {
            pos = pos + li.generate(&mut buffer[pos..]);
        }
        if let Some(li) = &self.sai {
            pos = pos + li.generate(&mut buffer[pos..]);
        }
        if let Some(li) = &self.rai {
            pos = pos + li.generate(&mut buffer[pos..]);
        }
        if let Some(li) = &self.tai {
            pos = pos + li.generate(&mut buffer[pos..]);
        }
        if let Some(li) = &self.ecgi {
            pos = pos + li.generate(&mut buffer[pos..]);
        }
        if let Some(li) = &self.lai {
            pos = pos + li.generate(&mut buffer[pos..]);
        }
        if let Some(li) = &self.menbid {
            pos = pos + li.generate(&mut buffer[pos..]);
        }
        if let Some(li) = &self.emenbid {
            pos = pos + li.generate(&mut buffer[pos..]);
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
            Some(CGI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
                0x4321,
            )),
            Some(SAI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
                0x4321,
            )),
            Some(RAI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
                0x4321,
            )),
            Some(TAI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
            )),
            Some(ECGI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0xF_FF_FF_FF
            ).unwrap()),
            Some(LAI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
            )),
            Some(MeNBID::new(
                PLMN::new([5,0,5], [0,9,9]),
                0xFFFFF,
            ).unwrap()),
            Some(EMeNBID::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1FFFFF,
            ).unwrap()),
            0).unwrap();

        let pos = ie.generate(&mut buffer);

        let expected = [InformationElementType::UserLocationInformation as u8,
            0, 51, // Length
            0, // Spare
            0b1111_1111, // Flags
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, 0x43, 0x21, // LAC & CI
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, 0x43, 0x21, // LAC & SAC
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, 0x43, 0x21, // LAC & RAC
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, // TAC
            0x05, 0x95, 0x90, // PLMN
            0x0F, 0xFF, 0xFF, 0xFF, // ECI
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, // LAC
            0x05, 0x95, 0x90, // PLMN
            0x0F, 0xFF, 0xFF, // MeNBID
            0x05, 0x95, 0x90, // PLMN
            0x1F, 0xFF, 0xFF, // MeNBID
        ];

        for i in 0..pos {
            if buffer[i] != expected[i] {
                println!("{} (actual) != {} (expected) at byte {}", buffer[i], expected[i], i);
                assert!(false);
            } 
        }
    }

    #[test]
    fn test_length() {
        let ie = InformationElement::new(
            Some(CGI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
                0x4321,
            )),
            Some(SAI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
                0x4321,
            )),
            Some(RAI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
                0x4321,
            )),
            Some(TAI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
            )),
            Some(ECGI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1FFFF
            ).unwrap()),
            Some(LAI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1234,
            )),
            Some(MeNBID::new(
                PLMN::new([5,0,5], [0,9,9]),
                0xFFFFF,
            ).unwrap()),
            Some(EMeNBID::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1FFFFF,
            ).unwrap()),
            0).unwrap();

        assert_eq!(ie.length(), 51+4);
    }

    #[test]
    fn test_message_type() {
        let ie = InformationElement::new(
            None,
            None,
            None,
            None,
            Some(ECGI::new(
                PLMN::new([5,0,5], [0,9,9]),
                0x1FFFF
            ).unwrap()),
            None,
            None,
            None,
            0).unwrap();

        assert_eq!(ie.information_element_type() as u8, InformationElementType::UserLocationInformation as u8)
    }

    #[test]
    fn test_message_parse() {
        let ie_bytes = [InformationElementType::UserLocationInformation as u8,
            0, 51, // Length
            0, // Spare
            0b1111_1111, // Flags
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, 0x43, 0x21, // LAC & CI
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, 0x43, 0x21, // LAC & SAC
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, 0x43, 0x21, // LAC & RAC
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, // TAC
            0x05, 0x95, 0x90, // PLMN
            0x0F, 0xFF, 0xFF, 0xFF, // ECI
            0x05, 0x95, 0x90, // PLMN
            0x12, 0x34, // LAC
            0x05, 0x95, 0x90, // PLMN
            0x0F, 0xFF, 0xFF, // MeNBID
            0x05, 0x95, 0x90, // PLMN
            0x1F, 0xFF, 0xFF, // MeNBID
        ];

        if let Some((ie, _pos)) = InformationElement::parse(&ie_bytes) {
            // Parsing was successful
            if let Some(li) = ie.cgi 
            {
                assert_eq!(li.plmn, PLMN::new([5,0,5], [0,9,9]));
                assert_eq!(li.lac, 0x1234);
                assert_eq!(li.ci, 0x4321);
            }
            else { assert!(false); }

            if let Some(li) = ie.sai 
            {
                assert_eq!(li.plmn, PLMN::new([5,0,5], [0,9,9]));
                assert_eq!(li.lac, 0x1234);
                assert_eq!(li.sac, 0x4321);
            }
            else { assert!(false); } 

            if let Some(li) = ie.rai 
            {
                assert_eq!(li.plmn, PLMN::new([5,0,5], [0,9,9]));
                assert_eq!(li.lac, 0x1234);
                assert_eq!(li.rac, 0x4321);
            }
            else { assert!(false); } 

            if let Some(li) = ie.tai 
            {
                assert_eq!(li.plmn, PLMN::new([5,0,5], [0,9,9]));
                assert_eq!(li.tac, 0x1234);
            }
            else { assert!(false); } 

            if let Some(li) = ie.ecgi 
            {
                assert_eq!(li.plmn, PLMN::new([5,0,5], [0,9,9]));
                assert_eq!(li.eci, 0xF_FF_FF_FF);
            }
            else { assert!(false); } 

            if let Some(li) = ie.lai 
            {
                assert_eq!(li.plmn, PLMN::new([5,0,5], [0,9,9]));
                assert_eq!(li.lac, 0x1234);
            }
            else { assert!(false); } 

            if let Some(li) = ie.menbid 
            {
                assert_eq!(li.plmn, PLMN::new([5,0,5], [0,9,9]));
                assert_eq!(li.menbid, 0xFFFFF);
            }
            else { assert!(false); }

            if let Some(li) = ie.emenbid 
            {
                assert_eq!(li.plmn, PLMN::new([5,0,5], [0,9,9]));
                assert_eq!(li.menbid, 0x1FFFFF);
            }
            else { assert!(false); } 
        }
        else {
            assert!(false);
        }
    }
}