use num_enum::FromPrimitive;

use super::{anc::AncStatus, cardle_status::ChargeStatus, peer_state::PeerState};

#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum NotificationType {
    Ancs = 0x01,
    #[default]
    Smthg = 0x00,
    PeerState = 0x02,
    ChargeStatus = 0x03,
    Message = 0x04,
    Control = 0x05,
    SystemPDL = 0x07,
}

#[derive(Debug, Clone, Copy)]
pub struct Smth {}

#[derive(Debug, Clone)]
pub enum Notification {
    Ancs(AncStatus),
    ChargeStatus(ChargeStatus),
    PeerState(PeerState),
    Smth(Smth),
    Message(String),
    Control(),
    UniverseConnected,
}

impl Notification {
    pub fn parse(command: u8, mut data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = command.try_into().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown NotificationType received: {e}"),
            )
        })?;

        Ok(match cmd {
            NotificationType::Ancs => Notification::Ancs(AncStatus::parse(data)?),
            NotificationType::PeerState => Notification::PeerState(PeerState::parse(data)?),
            NotificationType::Smthg => todo!(),
            NotificationType::ChargeStatus => {
                Notification::ChargeStatus(ChargeStatus::parse(data)?)
            }
            NotificationType::Message => {
                let mut buf = String::new();
                data.read_to_string(&mut buf)?;
                Notification::Message(buf)
            }
            NotificationType::Control => Notification::Control(),
            NotificationType::SystemPDL => Notification::UniverseConnected,
        })
    }
}
