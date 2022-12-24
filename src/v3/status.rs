use crate::byte_utils::extract_bits;

use super::packet_type::PacketType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub feature: u8,
    pub p_type: PacketType,
    pub cmd: u8,
    pub sealed: u16,
}

fn seal(feature: u8, p_type: PacketType, cmd: u8) -> u16 {
    ((feature as u16) << 9) + ((p_type as u16) << 7) + cmd as u16
}

impl Status {
    pub fn new(feature: u8, p_type: PacketType, cmd: u8) -> Self {
        Self {
            feature,
            p_type,
            cmd,
            sealed: seal(feature, p_type, cmd),
        }
    }

    pub fn sealed(&self) -> u16 {
        self.sealed
    }

    pub fn unseal(seal: u16) -> Self {
        Self {
            feature: extract_bits(seal, 9, 7),
            p_type: PacketType::try_from(extract_bits(seal, 7, 2)).unwrap(),
            cmd: extract_bits(seal, 0, 7),
            sealed: seal,
        }
    }
}
