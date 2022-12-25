use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    GetChargingState = 0x01,
    GetEq = 0x02,
    GetCustomEq = 0x03,
    GetAncAmbientSoundMode = 0x04,
    GetAptState = 0x05,
    GetGameMode = 0x06,

    GetWhisperMode = 0x08,

    GetAutoPlay = 0x0B,
    GetSerialNumber = 0x0C,

    GetTouchpadSettingLeft = 0x0E,
    GetTouchpadSettingRight = 0x0F,

    GetTouchpadLockState = 0x11,

    SetAncAutoAmbientSoundMode = 0x13,
    GetAncControlMode = 0x14,
    GetDolbyEq = 0x15,
    GetUniverse = 0x16,
    GetPDL = 0x17,

    GetFwVersion = 0x1F,

    GetPeerState = 0x21,

    GetAlwaysUvnanoMode = 0x2A,
    GetInEarState = 0x2B,
    GetAptxAdMode = 0x2C,
    GetA2DPBarge = 0x2D,
    GetRegionEq = 0x2E,

    SetEq = 0x42,
    SetCustomEq = 0x43,
    SetAncAmbientSoundMode = 0x44,
    SetAptxState = 0x45,
    SetGameMode = 0x46,

    SetWhisperMode = 0x48,

    SetVoiceNotification = 0x4A,
    SetAutoPlay = 0x4B,

    SetTouchpadSettingLeft = 0x4E,
    SetTouchpadSettingRight = 0x4F,
    SetFindDevice = 0x50,
    SetTouchpadLockState = 0x51,

    SetAncAutoAmbientMode = 0x53,
    SetAncControlMode = 0x54,
    SetDolbyEq = 0x55,
    SetUniverse = 0x56,

    SetAlwaysUvnanoMode = 0x6A,

    SetAptAdMode = 0x6C,
    SetA2DPBarge = 0x6D,
    SetRegionEq = 0x6E,
    SetRegionEq2 = 0x6F,
}
