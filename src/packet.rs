use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::byte_utils::ReadTail;

use super::{
    traits::Payload,
    v2::input::V1Packet,
    v3::packet::V3Packet,
    vendor_id::{V1V2_VENDOR_ID, V3_VENDOR_ID},
};

#[derive(Debug, Clone)]
pub enum PacketKind {
    Request,
    Response,
    Event,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Packet {
    V1(V1Packet),
    V3(V3Packet),
    Unknown { vendor_id: u16, data: Vec<u8> },
}

impl Packet {
    pub fn kind(&self) -> PacketKind {
        match self {
            Packet::V1(data) => data.kind(),
            Packet::V3(data) => data.kind(),
            Packet::Unknown { .. } => PacketKind::Unknown,
        }
    }

    pub fn packet_id(&self) -> (u16, u16) {
        match self {
            Packet::V1(data) => (V1V2_VENDOR_ID, data.packet_id()),
            Packet::Unknown { vendor_id, .. } => (*vendor_id, 0),
            Packet::V3(data) => (V3_VENDOR_ID, data.packet_id()),
        }
    }
}

impl Payload for Packet {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let vendor = data.read_u16::<BigEndian>()?;

        Ok(match vendor {
            V1V2_VENDOR_ID => Self::V1(V1Packet::read(data)?),
            V3_VENDOR_ID => Self::V3(V3Packet::read(data)?),
            vendor_id => Self::Unknown {
                vendor_id,
                data: data.read_tail()?,
            },
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Packet::V1(payload) => {
                buf.write_u16::<BigEndian>(V1V2_VENDOR_ID).unwrap();
                payload.write(&mut buf)?;
            }
            Packet::Unknown { vendor_id, data } => {
                buf.write_u16::<BigEndian>(*vendor_id).unwrap();
                data.write(&mut buf)?;
            }
            Packet::V3(data) => {
                buf.write_u16::<BigEndian>(V3_VENDOR_ID).unwrap();
                data.write(&mut buf)?;
            }
        };
        Ok(())
    }
}
