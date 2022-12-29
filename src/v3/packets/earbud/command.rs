use std::collections::HashMap;

use num_enum::TryFromPrimitive;

use super::{charging_status::ChargingStatus, fw_version::FwVersion, peer_state::PeerState, serial_number::SerialNumber};

use crate::v3::macros::request;

request!(Request Response Command {
    0x01 = GetChargingState(() => ChargingStatus),
    0x02 = GetEq(() => u8),
    0x03 = GetCustomEq(() => Vec<u8>),
    0x04 = GetAncAmbientSoundMode(() => Vec<u8>),
    0x05 = GetAptState(() => Vec<u8>),
    0x06 = GetGameMode(() => bool),

    0x08 = GetWhisperMode(() => bool),

    0x0B = GetAutoPlay(() => bool),
    0x0C = GetSerialNumber(() => SerialNumber),

    0x0E = GetTouchpadSettingLeft(() => HashMap<u8, u8>),
    0x0F = GetTouchpadSettingRight(() => HashMap<u8, u8>),

    0x11 = GetTouchpadLockState(() => bool),

    0x13 = GetAncAutoAmbientSoundMode(() => Vec<u8>), // check this
    0x14 = GetAncControlMode(() => bool),
    0x15 = GetDolbyEq(() => Vec<u8>),
    0x16 = GetUniverse(() => Vec<u8>),
    0x17 = GetPDL(() => Vec<u8>),

    0x1F = GetFwVersion(() => ([FwVersion; 2], [u8; 2])),

    0x21 = GetPeerState(() => PeerState),

    0x2A = GetAlwaysUvnanoMode(() => Vec<u8>),
    0x2B = GetInEarState(() => [bool; 2]),
    0x2C = GetAptxAdMode(() => Vec<u8>),
    0x2D = GetA2DPBarge(() => Vec<u8>),
    0x2E = GetRegionEq(() => Vec<u8>),

    0x42 = SetEq(Vec<u8> => ()),
    0x43 = SetCustomEq(Vec<u8> => ()),
    0x44 = SetAncAmbientSoundMode(Vec<u8> => ()),
    0x45 = SetAptxState(Vec<u8> => ()),
    0x46 = SetGameMode(Vec<u8> => ()),

    0x48 = SetWhisperMode(bool => ()),

    0x4A = SetVoiceNotification(Vec<u8> => ()),
    0x4B = SetAutoPlay(Vec<u8> => ()),

    0x4E = SetTouchpadSettingLeft(Vec<u8> => ()),
    0x4F = SetTouchpadSettingRight(Vec<u8> => ()),
    0x50 = SetFindDevice(Vec<u8> => ()),
    0x51 = SetTouchpadLockState(Vec<u8> => ()),

    0x53 = SetAncAutoAmbientSoundMode(Vec<u8> => ()),
    0x54 = SetAncControlMode(Vec<u8> => ()),
    0x55 = SetDolbyEq(Vec<u8> => ()),
    0x56 = SetUniverse(Vec<u8> => ()),

    0x6A = SetAlwaysUvnanoMode(Vec<u8> => ()),

    0x6C = SetAptAdMode(Vec<u8> => ()),
    0x6D = SetA2DPBarge(Vec<u8> => ()),
    0x6E = SetRegionEq(Vec<u8> => ()),
    0x6F = SetRegionEq2(Vec<u8> => ())
});
