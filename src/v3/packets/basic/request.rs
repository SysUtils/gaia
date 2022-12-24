use crate::{traits::Payload, v3::feature::QTILFeature};

use super::{
    command::Command,
    transport_info::{TransportInfo, TransportInfoType},
};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Request {
    FetchFeatures(),
    RegisterNotification(QTILFeature),
    GetTransportInfo(TransportInfoType),
    SetTransportParameter(TransportInfo),
}

impl Request {
    pub fn command(&self) -> Command {
        match self {
            Request::FetchFeatures() => Command::FetchFeatures,
            Request::RegisterNotification(_) => Command::RegisterNotification,
            Request::GetTransportInfo(_) => Command::GetTransportInfo,
            Request::SetTransportParameter(_) => Command::SetTransportParameter,
        }
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
