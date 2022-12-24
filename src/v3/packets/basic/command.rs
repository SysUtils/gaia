use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
pub enum Command {
    FetchFeatures = 0x01,
    RegisterNotification = 0x07,
    InitTransferData = 0x0A,
    GetTransportInfo = 0x0C,
    SetTransportParameter = 0x0D,
}
