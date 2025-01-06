use syn::parse::{Parse, ParseStream};

use super::scalar;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapField {
    pub name: String,
    pub key_ty: scalar::Ty,
    pub value_ty: ValueTy,
    pub tag: u32,
}

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValueTy {
    Scalar(scalar::Ty),
    Message(String),
}


impl Parse for ValueTy {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if fork.peek(syn::Ident) {
            if let Ok(ty) = fork.parse::<scalar::Ty>() {
                input.parse::<syn::Ident>()?;
                return Ok(ValueTy::Scalar(ty));
            }

            let ident = input.parse::<syn::Ident>()?;
            return Ok(ValueTy::Message(ident.to_string()));
        }
        Err(syn::Error::new(input.span(), "not a correct value type for map field"))
    }
}


impl Parse for MapField {

    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if fork.peek(syn::Ident) {
            let map_kw = fork.parse::<syn::Ident>()?;
            if map_kw.to_string().as_str() != "map" {
                return Err(syn::Error::new(map_kw.span(), "expected map keyword for map field"));
            }

            input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![<]>()?;

            let key_ty = input.parse::<scalar::Ty>()?;
            input.parse::<syn::Token![,]>()?;

            let value_ty = input.parse::<ValueTy>()?;
            input.parse::<syn::Token![>]>()?;

            let name = input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![=]>()?;

            let tag = input.parse::<syn::LitInt>()?;
            input.parse::<syn::Token![;]>()?;

            return Ok(MapField {
                name: name.to_string(),
                key_ty,
                value_ty,
                tag: tag.base10_parse::<u32>().unwrap(),
            });

        }

        Err(syn::Error::new(input.span(), "not a map field"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use quote::quote;

    proptest! {
        #[test]
        fn test_map_field_parsing(
            name in "[a-zA-Z][a-zA-Z0-9_]*",
            tag in 1..=100u32,
            // Key type must be scalar
            key_type in prop_oneof![
                Just("string"),
                Just("uint32"),
                Just("int32"),
            ],
            // Value type can be scalar or message
            value_type in prop_oneof![
                // Scalar types
                Just("string").prop_map(|s: &str| (s.to_string(), true)),
                Just("uint32").prop_map(|s: &str| (s.to_string(), true)),
                Just("bool").prop_map(|s| (s.to_string(), true)),
                // Message types (random valid identifier)
                "[A-Z][a-zA-Z0-9]*".prop_map(|s| (s.to_string(), false)),
            ],
        ) {
            let name_ident = syn::parse_str::<syn::Ident>(&name).unwrap();
            let key_ident = syn::parse_str::<syn::Ident>(&key_type).unwrap();
            let value_ident = syn::parse_str::<syn::Ident>(&value_type.0).unwrap();

            let input = quote!(
                map<#key_ident, #value_ident> #name_ident = #tag;
            );

            let field = syn::parse2::<MapField>(input).unwrap();

            prop_assert_eq!(field.name, name);
            prop_assert_eq!(field.tag, tag);
            prop_assert_eq!(field.key_ty, scalar::Ty::from_str(key_type).unwrap());

            // Check value type based on whether it's scalar or message
            if value_type.1 {
                // Scalar type
                prop_assert_eq!(
                    field.value_ty,
                    ValueTy::Scalar(scalar::Ty::from_str(&value_type.0).unwrap())
                );
            } else {
                // Message type
                prop_assert_eq!(
                    field.value_ty,
                    ValueTy::Message(value_type.0.to_string())
                );
            }
        }
    }
}
