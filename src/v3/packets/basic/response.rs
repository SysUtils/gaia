use std::collections::HashMap;

use crate::{
    byte_utils::ReadTail,
    {traits::Payload, v3::feature::QTILFeature},
};

use super::{command::Command, transport_info::TransportInfo};

#[derive(Debug, Clone)]
pub enum Response {
    FetchFeatures(HashMap<QTILFeature, u8>),
    GetTransportInfo(TransportInfo),
    SetTransportParameter(TransportInfo),
    RegisterNotification,
    Unknown(u8, Vec<u8>),
}

impl Response {
    pub fn command(&self) -> u8 {
        match self {
            Response::FetchFeatures(_) => Command::FetchFeatures as _,
            Response::GetTransportInfo(_) => Command::GetTransportInfo as _,
            Response::RegisterNotification => Command::RegisterNotification as _,
            Response::Unknown(command_id, _) => *command_id,
            Response::SetTransportParameter(_) => Command::SetTransportParameter as _,
        }
    }

    pub fn read(command: u8, data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = match Command::try_from(command) {
            Ok(o) => o,
            Err(cmd) => return Ok(Self::Unknown(cmd.number, data.read_tail()?)),
        };
        Ok(match cmd {
            Command::FetchFeatures => {
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
            Command::GetTransportInfo => Self::GetTransportInfo(TransportInfo::read(data)?),
            Command::SetTransportParameter => {
                Self::SetTransportParameter(TransportInfo::read(data)?)
            }
            Command::RegisterNotification => Self::RegisterNotification,
            Command::InitTransferData => todo!(),
        })
    }
}
