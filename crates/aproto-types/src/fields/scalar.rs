use core::fmt;

use anyhow::{anyhow, Error};
use proc_macro2::TokenStream;
use quote::quote;

/// A scalar protobuf field.
#[allow(unused)]
#[derive(Clone)]
pub struct Field {
    pub ty: Ty,
    pub kind: Kind,
    pub tag: u32,
}


#[allow(unused)]
#[derive(Clone, PartialEq, Eq)]
pub enum Ty {
    Double,
    Float,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Bool,
    String,
    Bytes(BytesTy),
}

#[allow(unused)]
impl Ty {
    /// Converts a protobuf type string into its corresponding `Ty` enum variant.
    ///
    /// This function is used to parse protobuf type definitions into their internal
    /// representation. It handles all scalar types defined in the protobuf specification.
    ///
    /// # Arguments
    /// * `s` - A string slice that represents the protobuf type (e.g., "uint32", "string")
    ///
    /// # Returns
    /// * `Ok(Ty)` - Successfully parsed type
    /// * `Err(Error)` - If the input string is not a valid protobuf type
    ///
    /// # Supported Types
    /// - Floating point: "float" (32-bit), "double" (64-bit)
    /// - Integers: "int32", "int64", "uint32", "uint64"
    /// - Boolean: "bool"
    /// - String: "string"
    /// - Bytes: "bytes" (converts to &[u8])
    ///
    /// # Note
    /// The function trims whitespace from the input string before matching,
    /// so " uint32 " is treated the same as "uint32".
    pub fn from_str(s: &str) -> Result<Ty, Error> {
        let err = Err(anyhow!("invalid type: {}", s));

        let ty = match s.trim() {
            "float" => Ty::Float,
            "double" => Ty::Double,
            "int32" => Ty::Int32,
            "int64" => Ty::Int64,
            "uint32" => Ty::Uint32,
            "uint64" => Ty::Uint64,
            "bool" => Ty::Bool,
            "string" => Ty::String,
            "bytes" => Ty::Bytes(BytesTy::Vec),
            _ => return err,
        };
        Ok(ty)
    }

    /// Converts a protobuf type to its corresponding Rust type representation as a TokenStream.
    ///
    /// The `quote!` macro generates Rust tokens that represent the type in the final generated code.
    /// For example:
    /// - `Ty::Uint32` becomes tokens representing `u32`
    /// - `Ty::String` becomes tokens representing `String`
    ///
    /// The `quote!` macro here converts each Rust type identifier into a TokenStream
    /// that can be used in code generation. This is essential for generating
    /// valid Rust struct fields from protobuf definitions
    pub fn rust_type(&self) -> TokenStream {
        match self {
            Ty::Double => quote!(f64),
            Ty::Float => quote!(f32),
            Ty::Int32 => quote!(i32),
            Ty::Int64 => quote!(i64),
            Ty::Uint32 => quote!(u32),
            Ty::Uint64 => quote!(u64),
            Ty::Bool => quote!(bool),
            Ty::String => quote!(String),
            Ty::Bytes(..) => quote!(&[u8]),
        }

    }

    /// Returns the type as it appears in protobuf field declarations
    pub fn as_str(&self) -> &'static str {
        match self {
            Ty::Double => "double",
            Ty::Float => "float",
            Ty::Int32 => "int32",
            Ty::Int64 => "int64",
            Ty::Uint32 => "uint32",
            Ty::Uint64 => "uint64",
            Ty::Bool => "bool",
            Ty::String => "string",
            Ty::Bytes(..) => "bytes",
        }
    }
}

impl fmt::Debug for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}


#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BytesTy {
    Vec,
    Bytes,
}

#[allow(unused)]
#[derive(Clone)]
pub enum Kind {
    Plain(DefaultValue),
    Optional(DefaultValue),
    Required(DefaultValue),
    Repeated,
}

/// Scalar Protobuf field Default Value
#[allow(unused)]
#[derive(Clone, Debug)]
pub enum DefaultValue {
    F64(f64),
    F32(f32),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    Bool(bool),
    String(String),
    Bytes(Vec<u8>),
}
