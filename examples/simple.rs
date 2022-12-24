use std::sync::Arc;

use bluer::{
    rfcomm::{Profile, ReqError, Role},
    AdapterEvent,
};
use futures::StreamExt;
use gaia::{
    packet::Packet,
    sender,
    transport::{bluez::Bluez, uuids::Service},
    v3,
};
use tracing::trace;
use tracing_subscriber::{fmt, prelude::*, util::SubscriberInitExt, EnvFilter};

async fn scan() -> Result<(), Box<dyn std::error::Error>> {
    let session = bluer::Session::new().await?;
    let dev = session.default_adapter().await.unwrap();
    let mut s = dev.discover_devices().await.unwrap();
    while let Some(AdapterEvent::DeviceAdded(addr)) = s.next().await {
        let d = dev.device(addr).unwrap();
        if d.is_connected().await.unwrap() {
            let uuid = d.uuids().await.unwrap().unwrap();
            let service = if uuid.contains(&Service::Spp.uuid()) {
                Service::Spp
            } else if uuid.contains(&Service::Gaia.uuid()) {
                Service::Gaia
            } else {
                Service::Spp
            };

            let mut p = session
                .register_profile(Profile {
                    uuid: service.uuid(),
                    name: Some("rfcat client".to_string()),
                    service: None,
                    role: Some(Role::Client),
                    require_authentication: Some(false),
                    require_authorization: Some(false),
                    auto_connect: Some(true),
                    ..Default::default()
                })
                .await
                .unwrap();
            let addr = d.address();
            tokio::spawn(async move { d.connect_profile(&service.uuid()).await.unwrap() });
            let socket = if let Some(req) = p.next().await {
                let req = req;
                trace!("connect request from {}", req.device());
                if req.device() == addr {
                    trace!("accepting request...{req:?}");
                    req.accept()?
                } else {
                    req.reject(ReqError::Rejected);
                    todo!()
                }
            } else {
                todo!()
            };

            let transport = Arc::new(Bluez::new(socket));
            let sender = sender::Sender::new(transport);
            let mut rx = sender.subscribe();

            dbg!(
                sender
                    .send(Packet::V3(v3::packet::V3Packet::Request(
                        v3::Request::Earbud(v3::earbud::Request::GetAutoPlay),
                    )))
                    .await
            );

            while let Ok(data) = rx.recv().await {
                dbg!("NEW EVENT", data);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "gaia=TRACE");
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    scan().await.unwrap();
}
