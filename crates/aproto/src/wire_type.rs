use aproto_types::error::AprotoError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
#[repr(u8)]
pub enum WireType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 3,
}

impl TryFrom<u64> for WireType {
    type Error = AprotoError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WireType::Varint),
            1 => Ok(WireType::Fixed64),
            2 => Ok(WireType::LengthDelimited),
            3 => Ok(WireType::Fixed32),
            _ => Err(AprotoError::InvalidWireType(value)),
        }
    }
}
