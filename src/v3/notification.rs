use crate::byte_utils::ReadTail;

use super::{
    packets::{anc, basic, earbud},
    QTILFeature,
};

#[derive(Debug, Clone)]
pub enum Notification {
    Anc(anc::Notification),
    Basic(basic::Notification),
    Earbud(earbud::Notification),
    Unknown { feature: u8, data: Vec<u8> },
}

impl Notification {
    pub fn command_id(&self) -> u16 {
        unimplemented!()
    }

    pub fn parse(feature: u8, command: u8, data: impl std::io::Read) -> std::io::Result<Self> {
        let feature = match QTILFeature::try_from(feature) {
            Ok(feature) => feature,
            Err(_) => {
                return Ok(Self::Unknown {
                    feature,
                    data: data.read_tail()?,
                })
            }
        };

        Ok(match feature {
            QTILFeature::Anc => Self::Anc(anc::Notification::parse(command, data)?),
            QTILFeature::Basic => Self::Basic(basic::Notification::parse(command, data)?),
            QTILFeature::Earbud => Self::Earbud(earbud::Notification::parse(command, data)?),
        })
    }
}
