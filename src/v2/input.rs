use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{packet::PacketKind, traits::Payload};

use super::{request::V1Request, response::V1Response};

const RESPONSE_FLAG: u16 = 0x8000;

#[derive(Debug, Clone)]
pub enum V1Packet {
    Request(V1Request),
    Response(V1Response),
}

impl V1Packet {
    pub fn kind(&self) -> PacketKind {
        match self {
            Self::Request(_) => PacketKind::Request,
            Self::Response(_) => PacketKind::Response,
        }
    }

    pub fn packet_id(&self) -> u16 {
        match self {
            V1Packet::Request(req) => req.command(),
            V1Packet::Response(resp) => resp.command(),
        }
    }
}

impl Payload for V1Packet {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = data.read_u16::<BigEndian>()?;
        let is_response = (cmd & RESPONSE_FLAG) != 0;
        let cmd = cmd & !RESPONSE_FLAG;

        Ok(if is_response {
            Self::Response(V1Response::read(cmd, data)?)
        } else {
            Self::Request(V1Request::read(cmd, data)?)
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        match self {
            V1Packet::Request(req) => {
                buf.write_u16::<BigEndian>(req.command())?;
                req.write(buf)?;
            }
            V1Packet::Response(resp) => {
                buf.write_u16::<BigEndian>(resp.command() & RESPONSE_FLAG)?;
                resp.write(buf)?;
            }
        }

        Ok(())
    }
}
