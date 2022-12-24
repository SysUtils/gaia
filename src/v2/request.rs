use crate::byte_utils::ReadTail;

use super::Command;

#[derive(Debug, Clone)]
pub enum V1Request {
    FetchVersion,
    Unknown { command: u16, data: Vec<u8> },
}

impl V1Request {
    pub fn command(&self) -> u16 {
        match self {
            Self::FetchVersion => Command::FetchVersion as _,
            Self::Unknown { command, .. } => *command,
        }
    }

    pub fn read(command: u16, data: impl std::io::Read) -> std::io::Result<Self> {
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
            Command::FetchVersion => Self::FetchVersion,
            _ => Self::Unknown {
                command,
                data: data.read_tail()?,
            },
        })
    }

    pub fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::FetchVersion => {}
            Self::Unknown { data, .. } => {
                buf.write_all(data.as_slice())?;
            }
        }
        Ok(())
    }
}
