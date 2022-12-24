use std::{borrow::Cow, fmt::Debug};

use bitflags::bitflags;

bitflags! {
    pub struct PacketFlags: u8 {
        const CHECKSUM = 0b00000001;
        const EXTENDED_LENGTH = 0b00000010;
    }
}

/// (len, ext)
fn payload_length(version: u16, mut length: u32) -> (u32, bool) {
    assert!(length >= 4, "invalid length");
    length -= 4;
    if length < 0xFE {
        return (length, false);
    }
    if version <= 3 {
        panic!("This version doesn't support length extension");
    }
    if length == 0xFF {
        return (length, false);
    }
    if length > 0xFFFF {
        panic!("Payload length bigger than maximum");
    }

    (length, true)
}

pub fn write_len(len: u32, buf: &mut [u8], ext: bool) {
    if ext {
        buf[3..5].copy_from_slice(&((len as u16).to_be_bytes()));
    } else {
        buf[3] = len as u8;
    }
}

pub fn read_len(buf: &[u8], ext: bool) -> u32 {
    if ext {
        u16::from_be_bytes(buf[3..5].try_into().unwrap()) as _
    } else {
        buf[3] as _
    }
}

pub fn full_len(len: u32, flags: PacketFlags) -> u32 {
    len + 8
        + flags.contains(PacketFlags::CHECKSUM) as u32
        + flags.contains(PacketFlags::EXTENDED_LENGTH) as u32
}

fn compute_checksum(_data: &[u8]) -> u8 {
    todo!()
}

#[derive(Debug)]
pub struct Packet<'a> {
    pub version: u16,
    pub checksum: bool,
    pub data: Cow<'a, [u8]>,
}

impl<'a> Packet<'a> {
    pub fn deserialize(_data: &[u8]) -> Packet<'static> {
        todo!()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let (l, e) = payload_length(self.version, self.data.len() as _);

        let mut flags = PacketFlags::empty();
        flags.set(PacketFlags::EXTENDED_LENGTH, e);

        let mut full_len = full_len(l, flags);
        let mut packet = vec![0u8; full_len as _];
        packet[0] = u8::MAX;
        packet[1] = self.version as _;
        packet[2] = flags.bits;

        write_len(l, packet.as_mut(), e);
        let offset = 4 + e as u32;
        packet[(offset as usize)..(offset + self.data.len() as u32) as _]
            .copy_from_slice(&self.data);
        if self.checksum {
            full_len -= 1;
            packet[full_len as usize] = compute_checksum(&packet[..full_len as usize]);
        }
        packet
    }
}
