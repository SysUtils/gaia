use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::traits;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TransferParameters {
    pub session: u16,
    pub offset: u32,
    pub bytes_left: u32,
}

impl traits::Payload for TransferParameters {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            session: data.read_u16::<BigEndian>()?,
            offset: data.read_u32::<BigEndian>()?,
            bytes_left: data.read_u32::<BigEndian>()?,
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_u16::<BigEndian>(self.session)?;
        buf.write_u32::<BigEndian>(self.offset)?;
        buf.write_u32::<BigEndian>(self.bytes_left)
    }
}
