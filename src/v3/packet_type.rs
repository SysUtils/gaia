use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum PacketType {
    Request = 0,
    Notification = 1,
    Response = 2,
    Error = 3,
}
