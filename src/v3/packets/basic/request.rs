use num_enum::TryFromPrimitive;

use crate::{traits::Payload, v3::feature::QTILFeature};

use super::protocol_info::{TransportInfo, TransportInfoType};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
pub enum CommandCode {
    FetchFeatures = 0x01,
    RegisterNotification = 0x07,
    InitTransferData = 0x0A,
    GetTransportInfo = 0x0C,
    SetTransportParameter = 0x0D,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Request {
    FetchFeatures(),
    RegisterNotification(QTILFeature),
    GetTransportInfo(TransportInfoType),
    SetTransportParameter(TransportInfo),
}

impl Request {
    pub fn code(&self) -> CommandCode {
        match self {
            Request::FetchFeatures() => CommandCode::FetchFeatures,
            Request::RegisterNotification(_) => CommandCode::RegisterNotification,
            Request::GetTransportInfo(_) => CommandCode::GetTransportInfo,
            Request::SetTransportParameter(_) => CommandCode::SetTransportParameter,
        }
    }

    pub fn command_id(&self) -> u8 {
        self.code() as _
    }
}

impl Payload for Request {
    fn read(_data: impl std::io::Read) -> std::io::Result<Self> {
        unimplemented!()
    }

    fn write(&self, buf: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Request::FetchFeatures() => vec![].write(buf),
            Request::RegisterNotification(n) => n.write(buf),
            Request::GetTransportInfo(n) => n.write(buf),
            Request::SetTransportParameter(n) => n.write(buf),
        }
    }
}
