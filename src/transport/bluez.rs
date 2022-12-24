use std::{borrow::Cow, fmt::Debug, sync::Arc};

use crate::{
    traits::{Payload, Transport},
    transport::common::{full_len, read_len, Packet, PacketFlags},
};
use async_trait::async_trait;
use bluer::rfcomm::{
    stream::{OwnedReadHalf, OwnedWriteHalf},
    Stream,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};

pub struct ReadFrame {
    read: OwnedReadHalf,
    data: Vec<u8>,
    len: usize,
}

pub struct Bluez {
    version: u16,
    write: Mutex<OwnedWriteHalf>,
    read: Mutex<ReadFrame>,
}

impl Bluez {
    pub fn new(stream: Stream) -> Self {
        let (r, w) = stream.into_split();
        Self {
            version: 1,
            read: Mutex::new(ReadFrame {
                read: r,
                data: vec![0; 0x10003],
                len: 0,
            }),
            write: Mutex::new(w),
        }
    }
}

#[async_trait]
impl Transport for Arc<Bluez> {
    #[tracing::instrument(skip(self), ret)]
    async fn send<T: Payload + Debug>(&self, pkt: T) {
        let mut data = Vec::new();
        pkt.write(&mut data).unwrap();
        let pkt = Packet {
            version: self.version,
            checksum: false,
            data: Cow::Owned(data),
        };
        self.write
            .lock()
            .await
            .write_all(&pkt.serialize())
            .await
            .unwrap();
    }

    #[tracing::instrument(skip(self), ret)]
    async fn receive(&self) -> Vec<u8> {
        let mut frame = self.read.lock().await;
        let ReadFrame {
            ref mut data,
            ref mut read,
            len,
        } = &mut *frame;
        loop {
            *len += read.read(data).await.unwrap();
            assert!(data[0] == u8::MAX);
            if *len < 3 {
                continue;
            }

            let flags = PacketFlags::from_bits_truncate(data[2]);
            let start_payload = 4 + flags.contains(PacketFlags::EXTENDED_LENGTH) as usize;
            if *len < start_payload {
                continue;
            }

            let payload_len = read_len(data, flags.contains(PacketFlags::EXTENDED_LENGTH));
            let pkt_len = full_len(payload_len, flags) as usize;
            if *len < pkt_len as _ {
                continue;
            }

            let mut res = frame.data[start_payload..pkt_len].to_vec();
            if flags.contains(PacketFlags::EXTENDED_LENGTH) {
                res.pop();
            }

            frame.data.drain(..pkt_len);

            return res;
        }
    }
}
