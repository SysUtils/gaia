use std::collections::HashMap;

use byteorder::ReadBytesExt;

use crate::byte_utils::ReadTail;

use super::{
    cardle_status::ChargeStatus, fw_version::FwVersion, peer_state::PeerState, request::CommandCode,
};

#[derive(Debug, Clone)]
pub enum Response {
    GetChargingState(ChargeStatus),
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
    pub fn parse(command: u8, mut data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = match CommandCode::try_from(command) {
            Ok(cmd) => cmd,
            _ => return Ok(Self::Unknown(command, data.read_tail()?)),
        };

        Ok(match cmd {
            CommandCode::GetEq => Self::GetEq(data.read_u8()?),
            CommandCode::GetCustomEq => Self::GetCustomEq(data.read_tail()?),
            CommandCode::GetAncAmbientSoundMode => Self::GetAncAmbientSoundMode(data.read_tail()?),
            CommandCode::GetAptState => Self::GetAptState(data.read_tail()?),
            CommandCode::GetGameMode => Self::GetGameMode(data.read_u8()? != 0),
            CommandCode::GetAutoPlay => Self::GetAutoPlay(data.read_u8()? != 0),
            CommandCode::GetSerialNumber => {
                Self::GetSerialNumber(String::from_utf8_lossy(&data.read_tail()?).into_owned())
            }
            CommandCode::GetTouchpadSettingLeft => Self::GetTouchpadSettingLeft(
                data.read_tail()?
                    .chunks_exact(2)
                    .map(|f| (f[0], f[1]))
                    .collect(),
            ),
            CommandCode::GetTouchpadSettingRight => Self::GetTouchpadSettingLeft(
                data.read_tail()?
                    .chunks_exact(2)
                    .map(|f| (f[0], f[1]))
                    .collect(),
            ),
            CommandCode::SetVoiceNotification => Self::SetVoiceNotification,
            CommandCode::SetAutoPlay => Self::SetAutoPlay,
            CommandCode::GetWhisperMode => Self::GetWhisperMode(data.read_u8()? != 0),
            CommandCode::GetTouchpadLockState => Self::GetTouchpadLockState(data.read_u8()? != 0),
            CommandCode::GetFwVersion => {
                let data = data.read_tail()?;
                Self::GetFwVersion((
                    [
                        FwVersion::try_from(&data[..4]).unwrap(),
                        FwVersion::try_from(&data[6..10]).unwrap(),
                    ],
                    data[12..14].try_into().unwrap(),
                ))
            }
            CommandCode::GetPeerState => Self::GetPeerState(PeerState::parse(data)?),
            CommandCode::SetWhisperMode => Self::SetWhisperMode,
            CommandCode::SetAncAutoAmbientSoundMode => Self::SetAncAutoAmbientMode,
            CommandCode::GetAncControlMode => Self::GetAncControlMode(data.read_u8()? != 0),
            CommandCode::GetDolbyEq => Self::GetDolbyEq(data.read_u8()?),
            CommandCode::GetUniverse => Self::GetUniverse(data.read_u8()?),
            CommandCode::GetPDL => Self::GetPDL,
            CommandCode::GetAlwaysUvnanoMode => Self::GetAlwaysUvnanoMode(data.read_u8()? != 0),
            CommandCode::GetInEarState => {
                Self::GetInEarState(((data.read_u8()? != 0), (data.read_u8()? != 0)))
            }
            CommandCode::GetAptxAdMode => Self::GetAptxAdMode(data.read_u8()?),
            CommandCode::GetA2DPBarge => Self::GetA2DPBarge(data.read_u8()?),
            CommandCode::GetRegionEq => Self::GetRegionEq(data.read_tail()?),
            CommandCode::SetEq => Self::SetEq,
            CommandCode::SetCustomEq => Self::SetCustomEq,
            CommandCode::SetAncAmbientSoundMode => Self::SetAncAmbientSoundMode,
            CommandCode::SetAptxState => Self::SetAptxState,
            CommandCode::SetGameMode => Self::SetGameMode,
            CommandCode::SetTouchpadSettingLeft => Self::SetTouchpadSettingLeft,
            CommandCode::SetTouchpadSettingRight => Self::SetTouchpadSettingRight,
            CommandCode::SetFindDevice => Self::SetFindDevice,
            CommandCode::SetToggleTouchPad => Self::SetToggleTouchPad,
            CommandCode::SetAncAutoAmbientMode => Self::SetAncAutoAmbientMode,
            CommandCode::SetAncControlMode => Self::SetAncControlMode,
            CommandCode::SetDolbyEq => Self::SetDolbyEq,
            CommandCode::SetUniverse => Self::SetUniverse,
            CommandCode::SetAlwaysUvnanoMode => Self::SetAlwaysUvnanoMode,
            CommandCode::SetAptAdMode => Self::SetAptAdMode,
            CommandCode::SetA2DPBarge => Self::SetA2DPBarge,
            CommandCode::SetRegionEq => Self::SetRegionEq,
            CommandCode::SetRegionEq2 => Self::SetRegionEq2,
            CommandCode::GetChargingState => Self::GetChargingState(ChargeStatus::parse(data)?),
        })
    }

    pub fn command_id(&self) -> u8 {
        match self {
            Response::GetChargingState(_) => CommandCode::GetChargingState as _,
            Response::GetEq(_) => CommandCode::GetEq as _,
            Response::GetCustomEq(_) => CommandCode::GetCustomEq as _,
            Response::GetAncAmbientSoundMode(_) => CommandCode::GetAncAmbientSoundMode as _,
            Response::GetAptState(_) => CommandCode::GetAptState as _,
            Response::GetGameMode(_) => CommandCode::GetGameMode as _,
            Response::GetAutoPlay(_) => CommandCode::GetAutoPlay as _,
            Response::GetSerialNumber(_) => CommandCode::GetSerialNumber as _,
            Response::GetTouchpadSettingLeft(_) => CommandCode::GetTouchpadSettingLeft as _,
            Response::SetVoiceNotification => CommandCode::SetVoiceNotification as _,
            Response::SetAutoPlay => CommandCode::SetAutoPlay as _,
            Response::GetWhisperMode(_) => CommandCode::GetWhisperMode as _,
            Response::GetTouchpadLockState(_) => CommandCode::GetTouchpadLockState as _,
            Response::GetFwVersion(_) => CommandCode::GetFwVersion as _,
            Response::GetPeerState(_) => CommandCode::GetPeerState as _,
            Response::SetWhisperMode => CommandCode::GetWhisperMode as _,
            Response::SetAncAutoAmbientMode => CommandCode::GetWhisperMode as _,
            Response::GetAncControlMode(_) => CommandCode::GetAncControlMode as _,
            Response::GetDolbyEq(_) => CommandCode::GetDolbyEq as _,
            Response::GetUniverse(_) => CommandCode::GetUniverse as _,
            Response::GetPDL => CommandCode::GetPDL as _,
            Response::GetAlwaysUvnanoMode(_) => CommandCode::GetAlwaysUvnanoMode as _,
            Response::GetInEarState(_) => CommandCode::GetInEarState as _,
            Response::GetAptxAdMode(_) => CommandCode::GetAptxAdMode as _,
            Response::GetA2DPBarge(_) => CommandCode::GetA2DPBarge as _,
            Response::GetRegionEq(_) => CommandCode::GetRegionEq as _,
            Response::SetEq => CommandCode::SetEq as _,
            Response::SetCustomEq => CommandCode::SetCustomEq as _,
            Response::SetAncAmbientSoundMode => CommandCode::SetAncAmbientSoundMode as _,
            Response::SetAptxState => CommandCode::SetAptxState as _,
            Response::SetGameMode => CommandCode::SetGameMode as _,
            Response::SetTouchpadSettingLeft => CommandCode::SetTouchpadSettingLeft as _,
            Response::SetTouchpadSettingRight => CommandCode::SetTouchpadSettingRight as _,
            Response::SetFindDevice => CommandCode::SetFindDevice as _,
            Response::SetToggleTouchPad => CommandCode::SetToggleTouchPad as _,
            Response::SetAncControlMode => CommandCode::SetAncControlMode as _,
            Response::SetDolbyEq => CommandCode::SetDolbyEq as _,
            Response::SetUniverse => CommandCode::SetUniverse as _,
            Response::SetAlwaysUvnanoMode => CommandCode::SetAlwaysUvnanoMode as _,
            Response::SetAptAdMode => CommandCode::SetAptAdMode as _,
            Response::SetA2DPBarge => CommandCode::SetA2DPBarge as _,
            Response::SetRegionEq => CommandCode::SetRegionEq as _,
            Response::SetRegionEq2 => CommandCode::SetRegionEq2 as _,
            Response::Unknown(code, _) => *code as _,
        }
    }
}
