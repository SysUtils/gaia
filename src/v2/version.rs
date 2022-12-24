use std::fmt::Display;

use byteorder::ReadBytesExt;

use crate::traits::Payload;

#[derive(Debug, Clone, Copy)]
pub struct ApiVersion {
    pub protocol: u8,
    pub minor: u8,
    pub major: u8,
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.protocol, self.major, self.minor)
    }
}

impl Payload for ApiVersion {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(ApiVersion {
            protocol: data.read_u8()?,
            minor: data.read_u8()?,
            major: data.read_u8()?,
        })
    }

    fn write(&self, mut data: impl std::io::Write) -> std::io::Result<()> {
        data.write_all(&[self.protocol, self.minor, self.major])
    }
}
