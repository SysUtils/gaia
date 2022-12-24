use std::io::ErrorKind;

use byteorder::ReadBytesExt;

#[derive(Debug, Clone, Copy)]
pub enum PeerState {
    Normal,
    LeftDisconnect,
    RightDisconnect,
}

impl PeerState {
    pub fn parse(mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(match data.read_u8()? {
            0 => Self::Normal,
            1 => Self::LeftDisconnect,
            2 => Self::RightDisconnect,
            _ => return Err(std::io::Error::new(ErrorKind::Other, "Invalid PeerState")),
        })
    }
}

impl TryFrom<&[u8]> for PeerState {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        Ok(match data[0] {
            0 => Self::Normal,
            1 => Self::LeftDisconnect,
            2 => Self::RightDisconnect,
            _ => return Err(()),
        })
    }
}
