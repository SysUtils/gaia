use num_enum::TryFromPrimitive;

use crate::v3::macros::notification;

notification!(Notification NotificationType {
    0x00 = ChargerStatus(bool)
});
