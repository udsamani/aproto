#[derive(thiserror::Error, Debug)]
pub enum AprotoError {
    #[error("invalid wire type: {0}")]
    InvalidWireType(u64),
}
