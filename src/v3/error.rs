use byteorder::ReadBytesExt;
use num_enum::FromPrimitive;

#[derive(Debug, Clone, Copy, FromPrimitive)]
#[repr(u8)]
pub enum ErrorType {
    FeatureSpecific = 0xFF,
    FeatureNotSupported = 0x00,
    CommandNotSupported = 0x01,
    NotAuthenticated = 0x02,
    InsufficientResources = 0x03,
    Authenticating = 0x04,
    InvalidParameter = 0x05,
    IncorrectState = 0x06,
    InProgress = 0x07,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Error {
    feature: u8,
    command: u8,

    error: ErrorType,
}

impl Error {
    pub fn full_command_id(&self) -> u16 {
        (self.feature as u16) << 8 | self.command as u16
    }

    pub fn parse(feature: u8, cmd: u8, mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            feature,
            command: cmd,
            error: data.read_u8()?.into(),
        })
    }
}
