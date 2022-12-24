use std::io::ErrorKind;

use crate::traits;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, TryFromPrimitive, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum TransportInfoType {
    MaxTxPacketSize = 0x01,
    OptimumTxPacketSize = 0x02,
    MaxRxPacketSize = 0x03,
    OptimumRxPacketSize = 0x04,
    TxFlowControl = 0x05,
    RxFlowControl = 0x06,
    ProtocolVersion = 0x07,
}

impl traits::Payload for TransportInfoType {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        data.read_u8()?.try_into().map_err(|e| {
            std::io::Error::new(
                ErrorKind::InvalidInput,
                format!("unknown transportn info type received: {e}"),
            )
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_u8(*self as _)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TransportInfo {
    pub info_type: TransportInfoType,
    pub value: u32,
}

impl traits::Payload for TransportInfo {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let t: TransportInfoType = data.read_u8()?.try_into().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid ProtocolInfoType received: {e}"),
            )
        })?;
        let v = data.read_u32::<BigEndian>()?;

        Ok(Self {
            info_type: t,
            value: v,
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_u8(self.info_type as _)?;
        buf.write_u32::<BigEndian>(self.value)
    }
}
