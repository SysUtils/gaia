use std::io::ErrorKind;

use byteorder::ReadBytesExt;

#[derive(Debug, Clone, Copy)]
pub enum PeerState {
    Normal,
    LeftDisconnect,
    RightDisconnect,
}

impl PeerState {
    pub fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(match data.read_u8()? {
            0 => Self::Normal,
            1 => Self::LeftDisconnect,
            2 => Self::RightDisconnect,
            _ => return Err(std::io::Error::new(ErrorKind::Other, "Invalid PeerState")),
        })
    }
}
