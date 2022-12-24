use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum EarbudPosition {
    Left,
    Right,
}
