mod scalar;
mod map;
mod message;

#[allow(unused)]
#[derive(Clone)]
pub enum Field {
    /// A scalar protobuf field.
    Scalar(scalar::Field),
    /// A message protobuf field.
    Message,
    /// A map protobuf field.
    Map(map::Field),
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    /// An optional field.
    Optional,
    /// A repeated field.
    Repeated,
}
