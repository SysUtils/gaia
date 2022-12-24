use crate::v3::Error;

use super::{notification::Notification, request::Request, response::Response};

pub enum AncPacket {
    Request(Request),
    Response(Response),
    Notification(Notification),
    Error(Error),
}
