use byteorder::{ReadBytesExt, WriteBytesExt};
use num_enum::TryFromPrimitive;

use crate::traits::Payload;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum EarbudPosition {
    Left,
    Right,
}

impl Payload for EarbudPosition {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        data.read_u8()?.try_into().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown EarbudPosition: {e}"),
            )
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_u8(*self as u8)
    }
}
