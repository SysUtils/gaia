use byteorder::ReadBytesExt;
use num_enum::TryFromPrimitive;

use super::earbud_position::EarbudPosition;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum NotificationType {
    State = 0x00,
    Mode = 0x01,
    LeakthroughGain = 0x02,
    AdaptiveState = 0x03,
    AdaptiveGain = 0x04,
}

#[derive(Debug, Clone, Copy)]
pub enum Notification {
    State(bool),
    Mode(u8), // WTF
    LeakthroughGain(u8),
    AdaptiveState((EarbudPosition, bool)),
    AdaptiveGain((EarbudPosition, u8)),
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
            NotificationType::State => Notification::State(data.read_u8()? != 0),
            NotificationType::Mode => Notification::Mode(data.read_u8()?),
            NotificationType::LeakthroughGain => Notification::LeakthroughGain(data.read_u8()?),
            NotificationType::AdaptiveState => {
                let pos = data.read_u8()?.try_into().map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unknown EarbudPosition received: {e}"),
                    )
                })?;
                let val = data.read_u8()? == 0;
                Notification::AdaptiveState((pos, val))
            }
            NotificationType::AdaptiveGain => {
                let pos = data.read_u8()?.try_into().map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unknown EarbudPosition received: {e}"),
                    )
                })?;
                let val = data.read_u8()?;
                Notification::AdaptiveGain((pos, val))
            }
        })
    }
}
