use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u16)]
pub enum Command {
    FetchVersion = 0x0300,
    UpgradeConnect = 0x1600,
    UpgradeDisconnect = 0x1601,
    UpgradeControl = 0x1602,
    RegisterFeature = 0x4001,
    Event = 0x4003,
}
