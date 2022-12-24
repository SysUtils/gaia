use std::io::ErrorKind;

use byteorder::{ReadBytesExt, WriteBytesExt};
use num_enum::TryFromPrimitive;

use crate::traits::Payload;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, TryFromPrimitive)]
pub enum QTILFeature {
    Basic = 0,
    Earbud = 1,
    Anc = 2,
    /*VoiceUI = 3,
    Debug = 4,
    MusicProcessing = 5,
    Upgrade = 6,
    HandsetService = 7,
    AudioCuration = 8,
    EarbudFit = 9,
    VoiceProcessing = 10,
    GestureConfiguration = 11,*/
}

impl Payload for QTILFeature {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        data.read_u8()?.try_into().map_err(|e| {
            std::io::Error::new(
                ErrorKind::InvalidInput,
                format!("unknown qtil feature received: {e}"),
            )
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_u8(*self as _)
    }
}
