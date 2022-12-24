use async_trait::async_trait;

pub enum PacketError {}

pub trait Payload: Sized + Send + Sync {
    fn read(data: impl std::io::Read) -> std::io::Result<Self>;
    fn write(&self, buf: impl std::io::Write) -> std::io::Result<()>;
}

impl Payload for Vec<u8> {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let mut res = vec![];
        data.read_to_end(&mut res)?;
        Ok(res)
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_all(self)
    }
}

#[async_trait]
pub trait Transport: Clone + Send + Sync {
    async fn receive(&self) -> Vec<u8>;
    async fn send<T: Payload + std::fmt::Debug>(&self, pkt: T);
}
