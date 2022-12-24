use std::str::FromStr;

use bluer::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum Service {
    Spp,
    Gaia,
}

impl Service {
    pub fn uuid(self) -> Uuid {
        match self {
            Service::Spp => Uuid::from_str("00001101-0000-1000-8000-00805F9B34FB").unwrap(),
            Service::Gaia => Uuid::from_str("00001107-D102-11E1-9B23-00025B00A5A5").unwrap(),
        }
    }
}
