use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive)]
#[repr(i8)]
pub enum Status {
    Empty = -2,
    Success = 0,
    CommandNotSupported = 1,
    NotAuthenticated = 2,
    InsufficientResources = 3,
    Authenticating = 4,
    InvalidParameter = 5,
    IncorrectState = 6,
    InProgress = 7,
}
