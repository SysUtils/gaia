pub fn extract_bits(data: u16, offset: u8, len: u8) -> u8 {
    assert!(offset + len <= 16);
    let mask = ((0x01u16 << len) - 1) << offset;
    ((data & mask) >> offset) as u8
}

pub trait ReadTail {
    fn read_tail(self) -> std::io::Result<Vec<u8>>;
}

impl<T: std::io::Read> ReadTail for T {
    fn read_tail(mut self) -> std::io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

pub trait ReadBool {
    fn read_bool(self) -> std::io::Result<bool>;
}

impl<T: byteorder::ReadBytesExt> ReadBool for T {
    fn read_bool(mut self) -> std::io::Result<bool> {
        Ok(match self.read_u8()? {
            0 => false,
            1 => true,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid bool representation",
                ))
            }
        })
    }
}
