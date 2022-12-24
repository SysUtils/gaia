#[derive(Debug, Clone)]
pub struct FwVersion(u8, u8, u8);

impl TryFrom<&[u8]> for FwVersion {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(FwVersion(value[0], value[2], value[3]))
    }
}
