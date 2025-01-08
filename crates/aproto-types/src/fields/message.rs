use syn::parse::{Parse, ParseStream};

use super::Label;

#[allow(unused)]
#[derive(Clone)]
pub struct MessageField {
    pub name: String,
    pub ty: String,
    pub label: Option<Label>,
    pub tag: u32,
}

impl Parse for MessageField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if fork.peek(syn::Ident) {
            let ident = fork.parse::<syn::Ident>()?;
            let label = Label::from_str(ident.to_string().as_str());
            if label.is_some() {
                input.parse::<syn::Ident>()?;
            }

            let ty = input.parse::<syn::Ident>()?;
            let name = input.parse::<syn::Ident>()?;
            input.parse::<syn::Token![=]>()?;
            let tag = input.parse::<syn::LitInt>()?.base10_parse::<u32>()?;
            input.parse::<syn::Token![;]>()?;

            return Ok(Self {
                name: name.to_string(),
                ty: ty.to_string(),
                label,
                tag,
            });
        }

        Err(syn::Error::new(input.span(), "expected message field"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use quote::quote;

    proptest! {
    #[test]
    fn test_parse_message_field(
        name in "[a-zA-Z][a-zA-Z0-9_]*",
        tag in 1..=100u32,
        ty in "[A-Z][a-zA-Z0-9]*",
        label in prop_oneof![
            Just("optional").prop_map(Some),
            Just("repeated").prop_map(Some),
            Just("").prop_map(|_| None),
        ]
    ) {
            let name_ident = syn::parse_str::<syn::Ident>(&name).unwrap();
            let ty_ident = syn::parse_str::<syn::Ident>(&ty).unwrap();

            let input = if let Some(label_str) = &label {
                let label_ident = syn::parse_str::<syn::Ident>(label_str).unwrap();
                quote!(#label_ident #ty_ident #name_ident = #tag;)
            } else {
                quote!(#ty_ident #name_ident = #tag;)
            };

            let expected_label = label.map(|l| match l {
                "optional" => Label::Optional,
                "repeated" => Label::Repeated,
                _ => unreachable!(),
            });

            let field = syn::parse2::<MessageField>(input).unwrap();
            assert_eq!(field.name, name);
            assert_eq!(field.ty, ty);
            assert_eq!(field.label, expected_label);
            assert_eq!(field.tag, tag);
        }
    }
}
