use crate::traits::Payload;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Request {}

impl Request {
    pub fn command(&self) -> u8 {
        unreachable!()
    }
}

impl Payload for Request {
    fn read(_data: impl std::io::Read) -> std::io::Result<Self> {
        unreachable!()
    }

    fn write(&self, _buf: impl std::io::Write) -> std::io::Result<()> {
        unreachable!()
    }
}
