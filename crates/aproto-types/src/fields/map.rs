use super::scalar;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    pub key_ty: scalar::Ty,
    pub value_ty: ValueTy,
    pub tag: u32,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueTy {
    Scalar(scalar::Ty),
}
