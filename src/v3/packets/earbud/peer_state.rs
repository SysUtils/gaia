use std::io::ErrorKind;

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::traits::Payload;

#[derive(Debug, Clone, Copy)]
pub enum PeerState {
    Normal,
    LeftDisconnect,
    RightDisconnect,
}

impl Payload for PeerState {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(match data.read_u8()? {
            0 => Self::Normal,
            1 => Self::LeftDisconnect,
            2 => Self::RightDisconnect,
            _ => return Err(std::io::Error::new(ErrorKind::Other, "Invalid PeerState")),
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_u8(match self {
            PeerState::Normal => 0,
            PeerState::LeftDisconnect => 1,
            PeerState::RightDisconnect => 2,
        })
    }
}
