use std::{collections::VecDeque, sync::Arc};

use dashmap::DashMap;
use tokio::sync::{broadcast, oneshot};

use crate::packet::Packet;

use super::traits::{Payload, Transport};

type RequestsMap = Arc<DashMap<(u16, u16), VecDeque<oneshot::Sender<Packet>>>>;

pub struct Sender<T: Transport> {
    transport: T,
    event_bus: tokio::sync::broadcast::Sender<Packet>,
    requests: RequestsMap,
}

impl<T: Transport + 'static> Sender<T> {
    pub fn new(transport: T) -> Self {
        let requests: RequestsMap = Default::default();
        let t1 = transport.clone();
        let r1 = requests.clone();
        let (tx, _) = broadcast::channel(1024);
        let event_bus = tx.clone();
        tokio::spawn(async move {
            loop {
                let data = t1.receive().await;
                let packet = match Packet::read(data.as_slice()) {
                    Ok(packet) => packet,
                    Err(e) => {
                        tracing::error!("packet deserialization error: {e}");
                        continue;
                    }
                };
                if packet.is_response() {
                    let tx = r1
                        .get_mut(&packet.command_id())
                        .and_then(|mut r| r.pop_front());
                    let tx = match tx {
                        Some(tx) => tx,
                        None => {
                            tracing::error!("Received packet without request, skip");
                            return;
                        }
                    };
                    let _ = tx.send(packet);
                } else if packet.is_event() {
                    let _ = event_bus.send(packet);
                } else {
                    tracing::error!("Unknown packet received: {:?}", packet)
                }
            }
        });
        Self {
            transport,
            event_bus: tx,

            requests,
        }
    }
    pub fn subscribe(&self) -> broadcast::Receiver<Packet> {
        self.event_bus.subscribe()
    }

    pub async fn send(&self, data: Packet) -> Packet {
        let id = data.command_id();
        let (tx, rx) = oneshot::channel();
        self.requests.entry(id).or_default().push_back(tx);
        self.transport.send(data).await;
        rx.await.unwrap()
    }
}
