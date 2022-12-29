use std::{array, collections::HashMap, hash::Hash, io::ErrorKind};

use async_trait::async_trait;
use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::byte_utils::ReadBool;

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

// TODO: use macro to implement Payload on tuples
impl<T: Payload> Payload for (T,) {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok((T::read(&mut data)?,))
    }

    fn write(&self, buf: impl std::io::Write) -> std::io::Result<()> {
        self.0.write(buf)?;
        Ok(())
    }
}

impl<T: Payload, T1: Payload> Payload for (T, T1) {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        Ok((T::read(&mut data)?, T1::read(&mut data)?))
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        self.0.write(&mut buf)?;
        self.1.write(&mut buf)?;
        Ok(())
    }
}

impl<const N: usize, T: Payload> Payload for [T; N] {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        // TODO: rewrite this
        array::try_from_fn(|_| T::read(&mut data))
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        for i in self {
            i.write(&mut buf)?;
        }
        Ok(())
    }
}

impl Payload for bool {
    fn read(data: impl std::io::Read) -> std::io::Result<Self> {
        data.read_bool()
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_u8(*self as u8)
    }
}

impl Payload for () {
    fn read(_data: impl std::io::Read) -> std::io::Result<Self> {
        Ok(())
    }

    fn write(&self, _buf: impl std::io::Write) -> std::io::Result<()> {
        Ok(())
    }
}

impl Payload for u8 {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        data.read_u8()
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        buf.write_u8(*self)
    }
}

impl<T: Payload + Eq + Hash, T1: Payload> Payload for HashMap<T, T1> {
    fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let mut res = HashMap::new();
        loop {
            let key = match T::read(&mut data) {
                Ok(key) => key,
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(res),
                Err(e) => return Err(e),
            };
            res.insert(key, T1::read(&mut data)?);
        }
    }

    fn write(&self, mut buf: impl std::io::Write) -> std::io::Result<()> {
        for (k, v) in self {
            k.write(&mut buf)?;
            v.write(&mut buf)?;
        }
        Ok(())
    }
}

#[async_trait]
pub trait Transport: Clone + Send + Sync {
    async fn receive(&self) -> Vec<u8>;
    async fn send<T: Payload + std::fmt::Debug>(&self, pkt: T);
}
