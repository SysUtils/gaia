use crate::{byte_utils::ReadTail, traits::Payload};

use super::QTILFeature;

#[derive(Debug, Clone)]
pub enum Request {
    Anc(super::anc::Request),
    Basic(super::basic::Request),
    Earbud(super::earbud::Request),
    Unknown {
        feature: u8,
        command: u8,
        data: Vec<u8>,
    },
}

impl Request {
    pub fn feature(&self) -> u8 {
        match self {
            Request::Anc(_) => QTILFeature::Anc as _,
            Request::Basic(_) => QTILFeature::Basic as _,
            Request::Earbud(_) => QTILFeature::Earbud as _,
            Request::Unknown { feature, .. } => *feature as _,
        }
    }

    pub fn packet_id(&self) -> u16 {
        match self {
            Request::Anc(p) => (QTILFeature::Anc as u16) << 8 | p.command() as u16,
            Request::Basic(p) => (QTILFeature::Basic as u16) << 8 | p.command() as u16,
            Request::Earbud(p) => (QTILFeature::Earbud as u16) << 8 | p.command() as u16,
            Request::Unknown {
                feature, command, ..
            } => (*feature as u16) << 8 | *command as u16,
        }
    }

    pub fn command(&self) -> u8 {
        match self {
            Request::Anc(p) => p.command(),
            Request::Basic(p) => p.command() as _,
            Request::Earbud(p) => p.command() as _,
            Request::Unknown { command, .. } => *command,
        }
    }

    pub fn write(&self, buf: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Request::Anc(a) => a.write(buf),
            Request::Basic(a) => a.write(buf),
            Request::Earbud(a) => a.write(buf),
            Request::Unknown { data, .. } => data.write(buf),
        }
    }

    pub fn read(feature: u8, command: u8, data: impl std::io::Read) -> std::io::Result<Self> {
        let feature = match QTILFeature::try_from(feature) {
            Ok(feature) => feature,
            Err(_) => {
                return Ok(Self::Unknown {
                    feature,
                    command,
                    data: data.read_tail()?,
                })
            }
        };

        match feature {
            QTILFeature::Anc => {}
            QTILFeature::Basic => todo!(),
            QTILFeature::Earbud => todo!(),
        }

        todo!()
    }
}
