use crate::traits;

use super::command::Command;

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
    pub fn command(&self) -> Command {
        match self {
            Request::GetChargingState => Command::GetChargingState,
            Request::GetEq => Command::GetEq,
            Request::GetCustomEq => Command::GetCustomEq,
            Request::GetAncAmbientSoundMode => Command::GetAncAmbientSoundMode,
            Request::GetAptState => Command::GetAptState,
            Request::GetGameMode => Command::GetGameMode,
            Request::GetWhisperMode => Command::GetWhisperMode,
            Request::GetAutoPlay => Command::GetAutoPlay,
            Request::GetSerialNumber => Command::GetSerialNumber,
            Request::GetTouchpadSettingLeft => Command::GetTouchpadSettingLeft,
            Request::GetTouchpadSettingRight => Command::GetTouchpadSettingRight,
            Request::GetTouchpadLockState => Command::GetTouchpadLockState,
            Request::GetAncControlMode => Command::GetAncControlMode,
            Request::GetDolbyEq => Command::GetDolbyEq,
            Request::GetUniverse => Command::GetUniverse,
            Request::GetPDL => Command::GetPDL,
            Request::GetFwVersion => Command::GetFwVersion,
            Request::GetPeerState => Command::GetPeerState,
            Request::GetAlwaysUvnanoMode => Command::GetAlwaysUvnanoMode,
            Request::GetInEarState => Command::GetInEarState,
            Request::GetAptxAdMode => Command::GetAptxAdMode,
            Request::GetA2DPBarge => Command::GetA2DPBarge,
            Request::GetRegionEq => Command::GetRegionEq,
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
