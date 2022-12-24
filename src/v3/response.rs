use crate::{
    byte_utils::ReadTail,
    v3::{anc, basic, earbud},
};

use super::QTILFeature;

#[derive(Debug, Clone)]
pub enum Response {
    Anc(super::anc::Response),
    Basic(super::basic::Response),
    Earbud(super::earbud::Response),
    Unknown { feature: u8, data: Vec<u8> },
}

impl Response {
    pub fn packet_id(&self) -> u16 {
        match self {
            Self::Anc(p) => (QTILFeature::Anc as u16) << 8 | p.command() as u16,
            Self::Basic(p) => (QTILFeature::Basic as u16) << 8 | p.command() as u16,
            Self::Earbud(p) => (QTILFeature::Earbud as u16) << 8 | p.command() as u16,
            Self::Unknown { feature, .. } => (*feature as u16) << 8,
        }
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
            QTILFeature::Anc => Self::Anc(anc::Response::read(command, data)?),
            QTILFeature::Basic => Self::Basic(basic::Response::read(command, data)?),
            QTILFeature::Earbud => Self::Earbud(earbud::Response::read(command, data)?),
        })
    }
}
