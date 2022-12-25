use std::collections::HashMap;

use byteorder::ReadBytesExt;

use crate::byte_utils::ReadTail;

use super::{
    charging_status::ChargingStatus, command::Command, fw_version::FwVersion, peer_state::PeerState,
};

#[derive(Debug, Clone)]
pub enum Response {
    GetChargingState(ChargingStatus),
    GetEq(u8),
    GetCustomEq(Vec<u8>),
    GetAncAmbientSoundMode(Vec<u8>),
    GetAptState(Vec<u8>),
    GetGameMode(bool),
    GetAutoPlay(bool),
    GetSerialNumber(String),
    GetTouchpadSettingLeft(HashMap<u8, u8>),
    SetVoiceNotification,
    SetAutoPlay,
    GetWhisperMode(bool),
    GetTouchpadLockState(bool),
    GetFwVersion(([FwVersion; 2], [u8; 2])),
    GetPeerState(PeerState),
    SetWhisperMode,
    SetAncAutoAmbientMode,
    GetAncControlMode(bool),
    GetDolbyEq(u8),
    GetUniverse(u8),
    GetPDL, // Not supported in my earbuds, so i can't test deserialization
    GetAlwaysUvnanoMode(bool),
    GetInEarState((bool, bool)),
    GetAptxAdMode(u8),
    GetA2DPBarge(u8),
    GetRegionEq(Vec<u8>),
    SetEq,
    SetCustomEq,
    SetAncAmbientSoundMode,
    SetAptxState,
    SetGameMode,
    SetTouchpadSettingLeft,
    SetTouchpadSettingRight,
    SetFindDevice,
    SetToggleTouchPad,
    SetAncControlMode,
    SetDolbyEq,
    SetUniverse,
    SetAlwaysUvnanoMode,
    SetAptAdMode,
    SetA2DPBarge,
    SetRegionEq,
    SetRegionEq2,
    Unknown(u8, Vec<u8>),
}

impl Response {
    pub fn read(command: u8, mut data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = match Command::try_from(command) {
            Ok(cmd) => cmd,
            _ => return Ok(Self::Unknown(command, data.read_tail()?)),
        };

        Ok(match cmd {
            Command::GetEq => Self::GetEq(data.read_u8()?),
            Command::GetCustomEq => Self::GetCustomEq(data.read_tail()?),
            Command::GetAncAmbientSoundMode => Self::GetAncAmbientSoundMode(data.read_tail()?),
            Command::GetAptState => Self::GetAptState(data.read_tail()?),
            Command::GetGameMode => Self::GetGameMode(data.read_u8()? != 0),
            Command::GetAutoPlay => Self::GetAutoPlay(data.read_u8()? != 0),
            Command::GetSerialNumber => {
                Self::GetSerialNumber(String::from_utf8_lossy(&data.read_tail()?).into_owned())
            }
            Command::GetTouchpadSettingLeft => Self::GetTouchpadSettingLeft(
                data.read_tail()?
                    .chunks_exact(2)
                    .map(|f| (f[0], f[1]))
                    .collect(),
            ),
            Command::GetTouchpadSettingRight => Self::GetTouchpadSettingLeft(
                data.read_tail()?
                    .chunks_exact(2)
                    .map(|f| (f[0], f[1]))
                    .collect(),
            ),
            Command::SetVoiceNotification => Self::SetVoiceNotification,
            Command::SetAutoPlay => Self::SetAutoPlay,
            Command::GetWhisperMode => Self::GetWhisperMode(data.read_u8()? != 0),
            Command::GetTouchpadLockState => Self::GetTouchpadLockState(data.read_u8()? != 0),
            Command::GetFwVersion => {
                let data = data.read_tail()?;
                Self::GetFwVersion((
                    [
                        FwVersion::try_from(&data[..4]).unwrap(),
                        FwVersion::try_from(&data[6..10]).unwrap(),
                    ],
                    data[12..14].try_into().unwrap(),
                ))
            }
            Command::GetPeerState => Self::GetPeerState(PeerState::read(data)?),
            Command::SetWhisperMode => Self::SetWhisperMode,
            Command::SetAncAutoAmbientSoundMode => Self::SetAncAutoAmbientMode,
            Command::GetAncControlMode => Self::GetAncControlMode(data.read_u8()? != 0),
            Command::GetDolbyEq => Self::GetDolbyEq(data.read_u8()?),
            Command::GetUniverse => Self::GetUniverse(data.read_u8()?),
            Command::GetPDL => Self::GetPDL,
            Command::GetAlwaysUvnanoMode => Self::GetAlwaysUvnanoMode(data.read_u8()? != 0),
            Command::GetInEarState => {
                Self::GetInEarState(((data.read_u8()? != 0), (data.read_u8()? != 0)))
            }
            Command::GetAptxAdMode => Self::GetAptxAdMode(data.read_u8()?),
            Command::GetA2DPBarge => Self::GetA2DPBarge(data.read_u8()?),
            Command::GetRegionEq => Self::GetRegionEq(data.read_tail()?),
            Command::SetEq => Self::SetEq,
            Command::SetCustomEq => Self::SetCustomEq,
            Command::SetAncAmbientSoundMode => Self::SetAncAmbientSoundMode,
            Command::SetAptxState => Self::SetAptxState,
            Command::SetGameMode => Self::SetGameMode,
            Command::SetTouchpadSettingLeft => Self::SetTouchpadSettingLeft,
            Command::SetTouchpadSettingRight => Self::SetTouchpadSettingRight,
            Command::SetFindDevice => Self::SetFindDevice,
            Command::SetTouchpadLockState => Self::SetToggleTouchPad,
            Command::SetAncAutoAmbientMode => Self::SetAncAutoAmbientMode,
            Command::SetAncControlMode => Self::SetAncControlMode,
            Command::SetDolbyEq => Self::SetDolbyEq,
            Command::SetUniverse => Self::SetUniverse,
            Command::SetAlwaysUvnanoMode => Self::SetAlwaysUvnanoMode,
            Command::SetAptAdMode => Self::SetAptAdMode,
            Command::SetA2DPBarge => Self::SetA2DPBarge,
            Command::SetRegionEq => Self::SetRegionEq,
            Command::SetRegionEq2 => Self::SetRegionEq2,
            Command::GetChargingState => Self::GetChargingState(ChargingStatus::read(data)?),
        })
    }

    pub fn command(&self) -> u8 {
        match self {
            Response::GetChargingState(_) => Command::GetChargingState as _,
            Response::GetEq(_) => Command::GetEq as _,
            Response::GetCustomEq(_) => Command::GetCustomEq as _,
            Response::GetAncAmbientSoundMode(_) => Command::GetAncAmbientSoundMode as _,
            Response::GetAptState(_) => Command::GetAptState as _,
            Response::GetGameMode(_) => Command::GetGameMode as _,
            Response::GetAutoPlay(_) => Command::GetAutoPlay as _,
            Response::GetSerialNumber(_) => Command::GetSerialNumber as _,
            Response::GetTouchpadSettingLeft(_) => Command::GetTouchpadSettingLeft as _,
            Response::SetVoiceNotification => Command::SetVoiceNotification as _,
            Response::SetAutoPlay => Command::SetAutoPlay as _,
            Response::GetWhisperMode(_) => Command::GetWhisperMode as _,
            Response::GetTouchpadLockState(_) => Command::GetTouchpadLockState as _,
            Response::GetFwVersion(_) => Command::GetFwVersion as _,
            Response::GetPeerState(_) => Command::GetPeerState as _,
            Response::SetWhisperMode => Command::SetWhisperMode as _,
            Response::SetAncAutoAmbientMode => Command::SetAncAutoAmbientMode as _,
            Response::GetAncControlMode(_) => Command::GetAncControlMode as _,
            Response::GetDolbyEq(_) => Command::GetDolbyEq as _,
            Response::GetUniverse(_) => Command::GetUniverse as _,
            Response::GetPDL => Command::GetPDL as _,
            Response::GetAlwaysUvnanoMode(_) => Command::GetAlwaysUvnanoMode as _,
            Response::GetInEarState(_) => Command::GetInEarState as _,
            Response::GetAptxAdMode(_) => Command::GetAptxAdMode as _,
            Response::GetA2DPBarge(_) => Command::GetA2DPBarge as _,
            Response::GetRegionEq(_) => Command::GetRegionEq as _,
            Response::SetEq => Command::SetEq as _,
            Response::SetCustomEq => Command::SetCustomEq as _,
            Response::SetAncAmbientSoundMode => Command::SetAncAmbientSoundMode as _,
            Response::SetAptxState => Command::SetAptxState as _,
            Response::SetGameMode => Command::SetGameMode as _,
            Response::SetTouchpadSettingLeft => Command::SetTouchpadSettingLeft as _,
            Response::SetTouchpadSettingRight => Command::SetTouchpadSettingRight as _,
            Response::SetFindDevice => Command::SetFindDevice as _,
            Response::SetToggleTouchPad => Command::SetTouchpadLockState as _,
            Response::SetAncControlMode => Command::SetAncControlMode as _,
            Response::SetDolbyEq => Command::SetDolbyEq as _,
            Response::SetUniverse => Command::SetUniverse as _,
            Response::SetAlwaysUvnanoMode => Command::SetAlwaysUvnanoMode as _,
            Response::SetAptAdMode => Command::SetAptAdMode as _,
            Response::SetA2DPBarge => Command::SetA2DPBarge as _,
            Response::SetRegionEq => Command::SetRegionEq as _,
            Response::SetRegionEq2 => Command::SetRegionEq2 as _,
            Response::Unknown(code, _) => *code as _,
        }
    }
}
