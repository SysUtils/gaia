use super::earbud_position::EarbudPosition;
use crate::v3::macros::notification;
use num_enum::TryFromPrimitive;

notification!(Notification NotificationType {
    0x00 = State(bool),
    0x01 = Mode(u8),
    0x02 = LeakthroughGain(u8),
    0x03 = AdaptiveState((EarbudPosition, bool)),
    0x04 = AdaptiveGain((EarbudPosition, u8))
});
