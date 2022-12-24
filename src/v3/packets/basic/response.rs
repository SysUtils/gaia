use std::collections::HashMap;

use crate::{
    byte_utils::ReadTail,
    {traits::Payload, v3::feature::QTILFeature},
};

use super::{protocol_info::TransportInfo, request::CommandCode};

#[derive(Debug, Clone)]
pub enum Response {
    FetchFeatures(HashMap<QTILFeature, u8>),
    GetTransportInfo(TransportInfo),
    SetTransportParameter(TransportInfo),
    RegisterNotification,
    Unknown(u8, Vec<u8>),
}

impl Response {
    pub fn code(&self) -> Option<CommandCode> {
        match self {
            Response::FetchFeatures(_) => Some(CommandCode::FetchFeatures),
            Response::GetTransportInfo(_) => Some(CommandCode::GetTransportInfo),
            Response::RegisterNotification => Some(CommandCode::RegisterNotification),
            Response::Unknown(_, _) => None,
            Response::SetTransportParameter(_) => Some(CommandCode::SetTransportParameter),
        }
    }

    pub fn command_id(&self) -> u8 {
        match self {
            Response::FetchFeatures(_) => CommandCode::FetchFeatures as _,
            Response::GetTransportInfo(_) => CommandCode::GetTransportInfo as _,
            Response::RegisterNotification => CommandCode::RegisterNotification as _,
            Response::Unknown(command_id, _) => *command_id,
            Response::SetTransportParameter(_) => CommandCode::SetTransportParameter as _,
        }
    }

    pub fn parse(command: u8, data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = match CommandCode::try_from(command) {
            Ok(o) => o,
            Err(cmd) => return Ok(Self::Unknown(cmd.number, data.read_tail()?)),
        };
        Ok(match cmd {
            CommandCode::FetchFeatures => {
                let d = data.read_tail()?;
                let features = d[1..]
                    .chunks_exact(2)
                    .flat_map(|data| match QTILFeature::try_from(data[0]) {
                        Ok(t) => Some((t, data[1])),
                        Err(e) => {
                            tracing::error!("unknown QTILFeature received: {e}");
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>();
                Self::FetchFeatures(features)
            }
            CommandCode::GetTransportInfo => Self::GetTransportInfo(TransportInfo::read(data)?),
            CommandCode::SetTransportParameter => {
                Self::SetTransportParameter(TransportInfo::read(data)?)
            }
            CommandCode::RegisterNotification => Self::RegisterNotification,
            CommandCode::InitTransferData => todo!(),
        })
    }
}
