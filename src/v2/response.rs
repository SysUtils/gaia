use byteorder::ReadBytesExt;

use crate::{byte_utils::ReadTail, traits::Payload};

use super::{version::ApiVersion, Command};

const EMPTY_EVENT: u16 = 0x0100;

#[derive(Debug, Clone)]
pub enum V1Response {
    FetchVersion { version: ApiVersion },
    UpgradeConnect {},
    UpgradeDisconnect {},
    UpgradeControl {},
    RegisterFeature {},
    Event { id: u16, data: Vec<u8> },
    Unknown { command: u16, data: Vec<u8> },
}

impl V1Response {
    pub fn command(&self) -> u16 {
        match self {
            Self::Event { .. } => Command::Event as _,
            Self::FetchVersion { .. } => Command::FetchVersion as _,
            Self::UpgradeConnect {} => Command::UpgradeConnect as _,
            Self::UpgradeDisconnect {} => Command::UpgradeDisconnect as _,
            Self::UpgradeControl {} => Command::UpgradeControl as _,
            Self::RegisterFeature {} => Command::RegisterFeature as _,
            Self::Unknown { command, .. } => *command,
        }
    }

    pub fn read(command: u16, mut data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = match Command::try_from(command) {
            Ok(cmd) => cmd,
            Err(_) => {
                return Ok(Self::Unknown {
                    command,
                    data: data.read_tail()?,
                });
            }
        };
        Ok(match cmd {
            Command::Event => {
                let id = data.read_u8().map(|i| i as u16).unwrap_or(EMPTY_EVENT);
                Self::Event {
                    id,
                    data: data.read_tail()?,
                }
            }
            Command::FetchVersion => Self::FetchVersion {
                version: ApiVersion::read(data)?,
            },
            Command::UpgradeConnect => Self::UpgradeConnect {},
            Command::UpgradeDisconnect => Self::UpgradeDisconnect {},
            Command::UpgradeControl => Self::UpgradeControl {},
            Command::RegisterFeature => Self::RegisterFeature {},
        })
    }

    pub fn write(&self, _buf: impl std::io::Write) -> std::io::Result<()> {
        unimplemented!()
    }
}
