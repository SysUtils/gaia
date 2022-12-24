#[derive(Debug, Clone, Copy)]
pub struct DeviceStatus {
    pub charging: bool,
    pub battery: u8,
}

impl From<&[u8]> for DeviceStatus {
    fn from(data: &[u8]) -> Self {
        let c = data[0] == 1;
        let stat = data[1];
        DeviceStatus {
            charging: c,
            battery: stat,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChargeStatus {
    pub left: DeviceStatus,
    pub right: DeviceStatus,
    pub cardle: DeviceStatus,
}
impl ChargeStatus {
    pub fn parse(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let buf = &mut [0u8; 6];
        data.read_exact(&mut buf[..])?;

        Ok(Self {
            left: DeviceStatus::from(&buf[0..2]),
            right: DeviceStatus::from(&buf[2..4]),
            cardle: DeviceStatus::from(&buf[4..]),
        })
    }
}

impl TryFrom<&[u8]> for ChargeStatus {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() != 6 {
            return Err(());
        }
        Ok(Self {
            left: DeviceStatus::from(&data[0..2]),
            right: DeviceStatus::from(&data[2..4]),
            cardle: DeviceStatus::from(&data[4..]),
        })
    }
}
