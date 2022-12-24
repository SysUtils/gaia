use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::traits::Payload;

use super::{request::V1Request, response::V1Response};

const RESPONSE_FLAG: u16 = 0x8000;

#[derive(Debug, Clone)]
pub enum V1Packet {
    Request(V1Request),
    Response(V1Response),
}

impl V1Packet {
    pub fn is_event(&self) -> bool {
        matches!(self, V1Packet::Response(V1Response::Event { .. }))
    }

    pub fn is_response(&self) -> bool {
        match self {
            V1Packet::Request(_) => false,
            V1Packet::Response(_) => true,
        }
    }

    pub fn command_id(&self) -> u16 {
        match self {
            V1Packet::Request(req) => req.code(),
            V1Packet::Response(resp) => resp.code(),
        }
    }
}

impl Payload for V1Packet {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = data.read_u16::<BigEndian>()?;
        let is_response = (cmd & RESPONSE_FLAG) != 0;
        let cmd = cmd & !RESPONSE_FLAG;

        Ok(if is_response {
            Self::Response(V1Response::parse(cmd, data)?)
        } else {
            Self::Request(V1Request::parse(cmd, data)?)
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        match self {
            V1Packet::Request(req) => {
                buf.write_u16::<BigEndian>(req.code())?;
                req.write(buf)?;
            }
            V1Packet::Response(resp) => {
                buf.write_u16::<BigEndian>(resp.code() & RESPONSE_FLAG)?;
                resp.write(buf)?;
            }
        }

        Ok(())
    }
}
