use std::collections::HashSet;

use syn::parse::{Parse, ParseStream};

mod map;
mod message;
mod scalar;
pub mod utils;

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
pub struct Fields(pub Vec<Field>);

impl Parse for Fields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut fields = Vec::new();
        let mut used_tags = HashSet::new();

        while !input.is_empty() {
            let proto_field: Field;
            if let Ok(field) = input.parse::<scalar::ScalarField>() {
                proto_field = Field::Scalar(field);
            } else if let Ok(field) = input.parse::<map::MapField>() {
                proto_field = Field::Map(field);
            } else if let Ok(field) = input.parse::<message::MessageField>() {
                proto_field = Field::Message(field);
            } else {
                return Err(syn::Error::new(input.span(), "expected a protobuf field"));
            }

            let tag = match &proto_field {
                Field::Scalar(field) => field.tag,
                Field::Map(field) => field.tag,
                Field::Message(field) => field.tag,
            };

            if used_tags.contains(&tag) {
                return Err(syn::Error::new(input.span(), "duplicate tag"));
            }
            used_tags.insert(tag);

            fields.push(proto_field);
        }
        Ok(Fields(fields))
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use quote::quote;
    use utils::{is_protobuf_reserve_key_word, is_rust_reserve_key_word};

    proptest! {
        #[test]
        fn test_mix_fields(
            (num_fields, field_types, names, labels, scalar_types, message_types) in (1..=100usize).prop_flat_map(|num_fields| {

                let is_valid_name = |name: &str| {
                    !is_protobuf_reserve_key_word(name) &&
                    !is_rust_reserve_key_word(name) &&
                    name.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) &&
                    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                };

                let valid_name_strategy = "[a-zA-Z][a-zA-Z0-9_]*"
                    .prop_filter("filtered out reserved words", move |s| is_valid_name(&s.clone()));

                (
                    Just(num_fields),
                    prop::collection::vec(prop_oneof![
                        "scalar",
                        "message",
                        "map"
                    ], num_fields),
                    prop::collection::vec(valid_name_strategy.clone(), num_fields),
                    prop::collection::vec(prop_oneof![
                        Just("optional").prop_map(Some),
                        Just("repeated").prop_map(Some),
                        Just("").prop_map(|_| None),
                    ], num_fields),
                    prop::collection::vec(prop_oneof![
                        "uint32",
                        "uint64",
                        "int32",
                        "int64",
                        "bool",
                        "string",
                        "bytes",
                    ], num_fields),
                    prop::collection::vec(valid_name_strategy, num_fields),
                )
            })
        ) {

            let mut tokens = quote!();
            for i in 0..num_fields {
                let field_type = &field_types[i];
                let tag = i as u32;
                let name = names[i].clone();
                let label = labels[i].clone();
                let scalar_type = &scalar_types[i];
                let message_type = &message_types[i];

                match field_type.as_str() {
                    "scalar" => {
                        let scalar_type_ident = syn::parse_str::<syn::Ident>(scalar_type).unwrap();
                        let name_ident = syn::parse_str::<syn::Ident>(&name).unwrap();
                        let input = if let Some(label_str) = &label {
                            let label_ident = syn::parse_str::<syn::Ident>(label_str).unwrap();
                            quote!(#label_ident #scalar_type_ident #name_ident = #tag;)
                        } else {
                            quote!(#scalar_type_ident #name_ident = #tag;)
                        };
                        tokens.extend(input);
                    },
                    "message" => {
                        let message_type_ident = syn::parse_str::<syn::Ident>(message_type).unwrap();
                        let name_ident = syn::parse_str::<syn::Ident>(&name).unwrap();
                        let input = if let Some(label_str) = &label {
                            let label_ident = syn::parse_str::<syn::Ident>(label_str).unwrap();
                            quote!(#label_ident #message_type_ident #name_ident = #tag;)
                        } else {
                            quote!(#message_type_ident #name_ident = #tag;)
                        };
                        tokens.extend(input);
                    },
                    "map" => {
                        let scalar_type_ident = syn::parse_str::<syn::Ident>(scalar_type).unwrap();
                        let name_ident = syn::parse_str::<syn::Ident>(&name).unwrap();
                        let input = quote!(map<#scalar_type_ident, #scalar_type_ident> #name_ident = #tag;);
                        tokens.extend(input);
                    },
                    _ => unreachable!(),
                }
            }

            let fields: Fields = syn::parse2(tokens).unwrap();
            assert_eq!(fields.0.len(), num_fields);

            for (i, field) in fields.0.iter().enumerate() {
                match field {
                    Field::Scalar(scalar) => {
                        assert_eq!(scalar.tag, i as u32);
                        assert_eq!(scalar.name, names[i]);
                        assert_eq!(scalar.ty, scalar::Ty::from_str(&scalar_types[i]).unwrap());
                        if let Some(label) = labels[i] {
                            assert_eq!(scalar.label, Label::from_str(label));
                        } else {
                            assert_eq!(scalar.label, None);
                        }
                    },
                    Field::Message(message) => {
                        assert_eq!(message.tag, i as u32);
                        assert_eq!(message.name, names[i]);
                        if let Some(label) = labels[i] {
                            assert_eq!(message.label, Label::from_str(label));
                        } else {
                            assert_eq!(message.label, None);
                        }
                        assert_eq!(message.ty, message_types[i]);
                    },
                    Field::Map(map) => {
                        assert_eq!(map.tag, i as u32);
                        assert_eq!(map.name, names[i]);
                        assert_eq!(map.key_ty, scalar::Ty::from_str(&scalar_types[i]).unwrap());
                    },
                }
            }
        }
    }
}
