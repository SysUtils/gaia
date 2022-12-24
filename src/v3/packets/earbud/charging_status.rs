#[derive(Debug, Clone, Copy)]
pub struct DeviceStatus {
    pub charging: bool,
    pub battery: u8,
}

impl DeviceStatus {
    fn from(data: &[u8]) -> Option<Self> {
        if data.len() != 2 {
            return None;
        }
        let c = data[0] == 1;
        let stat = data[1];

        if stat == 255 {
            None
        } else {
            Some(DeviceStatus {
                charging: c,
                battery: stat,
            })
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChargingStatus {
    pub left: Option<DeviceStatus>,
    pub right: Option<DeviceStatus>,
    pub cardle: Option<DeviceStatus>,
}

impl ChargingStatus {
    pub fn read(mut data: impl std::io::Read) -> std::io::Result<Self> {
        let buf = &mut [0u8; 6];
        data.read_exact(&mut buf[..])?;

        Ok(Self {
            left: DeviceStatus::from(&buf[0..2]),
            right: DeviceStatus::from(&buf[2..4]),
            cardle: DeviceStatus::from(&buf[4..]),
        })
    }
}
