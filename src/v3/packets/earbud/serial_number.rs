use crate::{byte_utils::ReadTail, traits::Payload};

#[derive(Debug, Clone)]
pub struct SerialNumber(String);

impl Payload for SerialNumber {
    fn read(data: impl std::io::Read) -> std::io::Result<Self> {
        let data = String::from_utf8(data.read_tail()?)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(Self(data))
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_all(self.0.as_bytes())
    }
}
