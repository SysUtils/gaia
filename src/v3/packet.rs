use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{packet::PacketKind, traits::Payload, v3};

use super::Status;

#[derive(Debug, Clone)]
pub enum V3Packet {
    Request(v3::request::Request),
    Notification(v3::notification::Notification),
    Response(v3::response::Response),
    Error(super::Error),
}

impl V3Packet {
    pub fn kind(&self) -> PacketKind {
        match self {
            Self::Request(_) => PacketKind::Request,
            Self::Response(_) => PacketKind::Response,
            Self::Notification(_) => PacketKind::Event,
            Self::Error(_) => PacketKind::Response,
        }
    }

    pub fn packet_id(&self) -> u16 {
        match self {
            V3Packet::Request(req) => req.packet_id(),
            V3Packet::Notification(_) => unimplemented!(),
            V3Packet::Response(resp) => resp.packet_id(),
            V3Packet::Error(e) => e.packet_id(),
        }
    }
}

impl Payload for V3Packet {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = Status::unseal(data.read_u16::<BigEndian>()?);
        Ok(match cmd.p_type {
            v3::PacketType::Request => todo!(),
            v3::PacketType::Notification => {
                Self::Notification(v3::Notification::read(cmd.feature, cmd.cmd, data)?)
            }
            v3::PacketType::Response => {
                Self::Response(v3::Response::parse(cmd.feature, cmd.cmd, data)?)
            }
            v3::PacketType::Error => Self::Error(super::Error::read(cmd.feature, cmd.cmd, data)?),
        })
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        match self {
            V3Packet::Request(r) => {
                let status = Status::new(r.feature(), v3::PacketType::Request, r.command());
                buf.write_u16::<BigEndian>(status.sealed())?;
                r.write(buf)?;
            }
            V3Packet::Notification(_) => todo!(),
            V3Packet::Response(_) => todo!(),
            V3Packet::Error(_) => todo!(),
        }
        Ok(())
    }
}
