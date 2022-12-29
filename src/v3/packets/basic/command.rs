use super::transport_info::{TransportInfo, TransportInfoType};
use crate::v3::macros::request;
use crate::v3::QTILFeature;
use num_enum::TryFromPrimitive;
use std::collections::HashMap;

request!(Request Response Command {
    0x00 = FetchFeatures(() => HashMap<QTILFeature, u8>),
    0x07 = RegisterNotification(QTILFeature => ()),
    0x0A = InitTransferData(() => ()),
    0x0C = GetTransportInfo(TransportInfoType => TransportInfo),
    0x0D = SetTransportParameter(TransportInfo => TransportInfo)
});
