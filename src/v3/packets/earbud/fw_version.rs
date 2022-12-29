use byteorder::ReadBytesExt;

use crate::traits::Payload;

#[derive(Debug, Clone)]
pub struct FwVersion(u8, u8, u8);

impl Payload for FwVersion {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(Self(data.read_u8()?, data.read_u8()?, data.read_u8()?))
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_all(&[self.0, self.1, self.2])
    }
}
