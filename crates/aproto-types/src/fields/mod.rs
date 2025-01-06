mod scalar;
mod map;
mod message;


#[allow(unused)]
#[derive(Clone)]
pub enum Field {
    /// A scalar protobuf field.
    Scalar(scalar::ScalarField),
    /// A message protobuf field.
    Message(message::MessageField),
    /// A map protobuf field.
    Map(map::MapField),
}


#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Label {
    /// An optional field.
    Optional,
    /// A repeated field.
    Repeated,
}

#[allow(clippy::should_implement_trait)]
impl Label {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "optional" => Some(Self::Optional),
            "repeated" => Some(Self::Repeated),
            _ => None,
        }
    }
}
