use num_enum::TryFromPrimitive;

use crate::traits::Payload;

use super::{anc::AncStatus, charging_status::ChargingStatus, peer_state::PeerState};

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum NotificationType {
    Ancs = 0x01,
    PeerState = 0x02,
    ChargeStatus = 0x03,
    Message = 0x04,
    Control = 0x05,
    SystemPDL = 0x07,
}

#[derive(Debug, Clone)]
pub enum Notification {
    Ancs(AncStatus),
    ChargeStatus(ChargingStatus),
    PeerState(PeerState),
    Message(String),
    Control(),
    UniverseConnected,
}

impl Notification {
    pub fn read(command: u8, mut data: impl std::io::Read) -> std::io::Result<Self> {
        let cmd = command.try_into().map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unknown NotificationType received: {e}"),
            )
        })?;

        Ok(match cmd {
            NotificationType::Ancs => Notification::Ancs(AncStatus::read(data)?),
            NotificationType::PeerState => Notification::PeerState(PeerState::read(data)?),
            NotificationType::ChargeStatus => {
                Notification::ChargeStatus(ChargingStatus::read(data)?)
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
