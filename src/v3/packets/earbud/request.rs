use num_enum::TryFromPrimitive;

use crate::traits;

#[derive(Debug, TryFromPrimitive, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum CommandCode {
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
    SetToggleTouchPad = 0x51,

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Request {
    GetChargingState,
    GetEq,
    GetCustomEq,
    GetAncAmbientSoundMode,
    GetAptState,
    GetGameMode,

    GetWhisperMode,

    GetAutoPlay,
    GetSerialNumber,

    GetTouchpadSettingLeft,
    GetTouchpadSettingRight,

    GetTouchpadLockState,

    GetAncControlMode,
    GetDolbyEq,
    GetUniverse,
    GetPDL,

    GetFwVersion,

    GetPeerState,

    GetAlwaysUvnanoMode,
    GetInEarState,
    GetAptxAdMode,
    GetA2DPBarge,
    GetRegionEq,
}

impl Request {
    pub fn command_id(self) -> u8 {
        self.code() as _
    }

    fn code(&self) -> CommandCode {
        match self {
            Request::GetChargingState => CommandCode::GetChargingState,
            Request::GetEq => CommandCode::GetEq,
            Request::GetCustomEq => CommandCode::GetCustomEq,
            Request::GetAncAmbientSoundMode => CommandCode::GetAncAmbientSoundMode,
            Request::GetAptState => CommandCode::GetAptState,
            Request::GetGameMode => CommandCode::GetGameMode,
            Request::GetWhisperMode => CommandCode::GetWhisperMode,
            Request::GetAutoPlay => CommandCode::GetAutoPlay,
            Request::GetSerialNumber => CommandCode::GetSerialNumber,
            Request::GetTouchpadSettingLeft => CommandCode::GetTouchpadSettingLeft,
            Request::GetTouchpadSettingRight => CommandCode::GetTouchpadSettingRight,
            Request::GetTouchpadLockState => CommandCode::GetTouchpadLockState,
            Request::GetAncControlMode => CommandCode::GetAncControlMode,
            Request::GetDolbyEq => CommandCode::GetDolbyEq,
            Request::GetUniverse => CommandCode::GetUniverse,
            Request::GetPDL => CommandCode::GetPDL,
            Request::GetFwVersion => CommandCode::GetFwVersion,
            Request::GetPeerState => CommandCode::GetPeerState,
            Request::GetAlwaysUvnanoMode => CommandCode::GetAlwaysUvnanoMode,
            Request::GetInEarState => CommandCode::GetInEarState,
            Request::GetAptxAdMode => CommandCode::GetAptxAdMode,
            Request::GetA2DPBarge => CommandCode::GetA2DPBarge,
            Request::GetRegionEq => CommandCode::GetRegionEq,
        }
    }
}

impl traits::Payload for Request {
    fn read(_data: impl std::io::Read) -> std::io::Result<Self> {
        todo!()
    }

    fn write(&self, _buf: impl std::io::Write) -> std::io::Result<()> {
        Ok(())
    }
}
