use std::fmt;
use anyhow::{anyhow, Error};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

use super::Label;

/// A scalar protobuf field.
#[allow(unused)]
#[derive(Clone)]
pub struct ScalarField {
    pub name: String,
    pub label: Option<Label>,
    pub ty: Ty,
    pub tag: u32,
}


impl ScalarField {

    pub fn is_scalar_field(input: &str) -> bool {
        let ty = Ty::from_str(input);
        ty.is_ok()
    }
}

impl Parse for ScalarField {
    fn parse(input: ParseStream) -> syn::Result<Self> {

        let fork = input.fork();
        let mut label = None;

        if input.peek(syn::Ident) {
            let ident = fork.parse::<syn::Ident>()?;
            label = match ident.to_string().as_str() {
                "optional" => {
                    input.parse::<syn::Ident>()?;
                    Some(Label::Optional)
                },
                "repeated" => {
                    input.parse::<syn::Ident>()?;
                    Some(Label::Repeated)
                },
                _ => None,
            };
        }

        let fork = input.fork();
        if input.peek(syn::Ident) {
            let ty: syn::Ident = fork.parse()?;
            if !ScalarField::is_scalar_field(&ty.to_string()) {
                return Err(syn::Error::new(input.span(), "not a scalar field"));
            }

            let ty = input.parse::<syn::Ident>()?;
            let ty = Ty::from_str(&ty.to_string()).map_err(|e| syn::Error::new(input.span(), e.to_string()))?;
            let name = input.parse::<syn::Ident>()?;
            let _ = input.parse::<syn::Token![=]>()?;
            let tag = input.parse::<syn::LitInt>()?;
            let tag = tag.base10_parse::<u32>()?;

            return Ok(ScalarField { name: name.to_string(), label, ty, tag });
        }
        Err(syn::Error::new(input.span(), "not a scalar field"))
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;


    proptest! {
        #[test]
        fn test_all_scalar_fields_with_optional(
            name in "[a-z][a-z0-9_]*",
            ty in prop_oneof!(
                Just("uint32"),
                Just("int32"),
                Just("bool"),
                Just("string"),
                Just("double"),
                Just("float"),
                Just("uint64"),
                Just("bytes"),
            ),
            tag in 1..=100u32,
            label in prop_oneof!(
                Just("optional"),
                Just("repeated"),
            ),
        ) {
            let name_ident = syn::parse_str::<syn::Ident>(&name).unwrap();
            let ty_ident = syn::parse_str::<syn::Ident>(&ty).unwrap();
            let label_ident = syn::parse_str::<syn::Ident>(&label).unwrap();

            let input = quote!(#label_ident #ty_ident #name_ident = #tag);
            let field = syn::parse2::<ScalarField>(input).unwrap();

            let expected_label = match label {
                "optional" => Some(Label::Optional),
                "repeated" => Some(Label::Repeated),
                _ => None,
            };

            assert_eq!(field.name, name);
            assert_eq!(field.tag, tag);
            assert_eq!(field.ty, Ty::from_str(&ty).unwrap());
            assert_eq!(field.label, expected_label);
        }

    }

}
