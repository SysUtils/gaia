use std::io::ErrorKind;

use crate::byte_utils::ReadTail;

#[derive(Debug, Clone, Copy)]
pub enum AncMode {
    Auto,
    High,
    Low,
}

#[derive(Debug, Clone, Copy)]
pub enum AncStatus {
    Off,
    On(AncMode),
    Extended(bool),
    Unknown,
}

impl AncStatus {
    pub fn parse(data: impl std::io::Read) -> std::io::Result<Self> {
        let data = data.read_tail()?;
        Ok(if data.len() == 4 {
            match data[..2] {
                [0, 0] => AncStatus::Off,               // AptStatus 0
                [1, 0] => AncStatus::On(AncMode::Auto), // AptStatus 1
                [1, 1] => {
                    let v = data[2] != 0;
                    AncStatus::Extended(v) // AptStatus 5, 6
                }
                _ => return Err(std::io::Error::new(ErrorKind::Other, "Invalid AncStatus")),
            }
        } else if data.len() == 5 {
            match data[..2] {
                [0, _] => AncStatus::Off, // AptStatusOff
                [1, 0] => match data[4] {
                    0 => AncStatus::On(AncMode::Auto),
                    1 => AncStatus::On(AncMode::High),
                    2 => AncStatus::On(AncMode::Low),
                    _ => return Err(std::io::Error::new(ErrorKind::Other, "Invalid AncStatus")),
                },
                [1, 1] => {
                    let v = data[2] != 0;
                    AncStatus::Extended(v)
                }
                _ => return Err(std::io::Error::new(ErrorKind::Other, "Invalid AncStatus")),
            }
        } else {
            Self::Unknown
        })
    }
}

impl TryFrom<&[u8]> for AncStatus {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(if data.len() == 4 {
            match data[..2] {
                [0, 0] => AncStatus::Off,               // AptStatus 0
                [1, 0] => AncStatus::On(AncMode::Auto), // AptStatus 1
                [1, 1] => {
                    let v = data[2] != 0;
                    AncStatus::Extended(v) // AptStatus 5, 6
                }
                _ => return Err(()),
            }
        } else if data.len() == 5 {
            match data[..2] {
                [0, _] => AncStatus::Off, // AptStatusOff
                [1, 0] => match data[4] {
                    0 => AncStatus::On(AncMode::Auto),
                    1 => AncStatus::On(AncMode::High),
                    2 => AncStatus::On(AncMode::Low),
                    _ => return Err(()),
                },
                [1, 1] => {
                    let v = data[2] != 0;
                    AncStatus::Extended(v)
                }
                _ => return Err(()),
            }
        } else {
            Self::Unknown
        })
    }
}
