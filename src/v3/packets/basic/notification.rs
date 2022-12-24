use byteorder::ReadBytesExt;
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum NotificationType {
    ChargerStatus = 0x00,
}

#[derive(Debug, Clone, Copy)]
pub enum Notification {
    ChargerStatus(bool),
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
            NotificationType::ChargerStatus => Notification::ChargerStatus(data.read_u8()? != 0),
        })
    }
}
