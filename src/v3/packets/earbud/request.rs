use byteorder::WriteBytesExt;

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

    SetAncAmbientSoundMode([u8; 2]),
    SetAmbientSoundParameter(u8),
    SetTouchpadLockState(bool),
    SetWhisperMode(bool),
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
            Request::SetAncAmbientSoundMode(_) => Command::SetAncAmbientSoundMode,
            Request::SetAmbientSoundParameter(_) => Command::SetAptxState,
            Request::SetTouchpadLockState(_) => Command::SetTouchpadLockState,
            Request::SetWhisperMode(_) => Command::SetWhisperMode,
        }
    }
}

impl traits::Payload for Request {
    fn read(_data: impl std::io::Read) -> std::io::Result<Self> {
        todo!()
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Request::SetAncAmbientSoundMode(data) => {
                buf.write_all(&data[..])?;
            }
            Request::SetAmbientSoundParameter(data) => {
                buf.write_u8(*data)?;
            }
            Request::SetTouchpadLockState(data) => {
                buf.write_u8(*data as _)?;
            }
            Request::SetWhisperMode(data) => {
                buf.write_u8((*data as u8) * 2)?;
            }
            _ => {}
        }
        Ok(())
    }
}
