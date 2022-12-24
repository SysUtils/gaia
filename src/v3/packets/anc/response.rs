use crate::byte_utils::ReadTail;

#[derive(Debug, Clone)]
pub enum Response {
    Unknown { command: u8, data: Vec<u8> },
}

impl Response {
    pub fn command_id(&self) -> u8 {
        match self {
            Response::Unknown { command, .. } => *command,
        }
    }
    pub fn parse(command: u8, data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(Response::Unknown {
            command,
            data: data.read_tail()?,
        })
    }
}
