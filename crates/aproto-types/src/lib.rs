mod fields;

pub use fields::*;
use syn::parse::{Parse, ParseStream};
use crate::fields::utils::is_protobuf_reserve_key_word;

#[allow(unused)]
pub struct ProtobufMessageDescriptor {
    pub name: String,
    pub fields: Fields
}


impl Parse for ProtobufMessageDescriptor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let message_name = input.parse::<syn::Ident>()?;
        if message_name != "message" {
            return Err(syn::Error::new(input.span(), "expected message keyword"));
        }
        let name = input.parse::<syn::Ident>()?;
        if is_protobuf_reserve_key_word(&name.to_string()) {
            return Err(syn::Error::new(input.span(), "reserved keyword"));
        }
        let content;
        syn::braced!(content in input);
        let fields = content.parse::<Fields>()?;
        Ok(Self { name: name.to_string(), fields })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;


    #[test]
    pub fn test_parse_message_descriptor() {
        let input = quote!(
            message TestMessage {
                string name = 1;
                int32 age = 2;
                repeated string hobbies = 3;
                map<string, int32> scores = 4;
            }
        );
        let message = syn::parse2::<ProtobufMessageDescriptor>(input).unwrap();
        assert_eq!(message.name, "TestMessage");
        assert_eq!(message.fields.0.len(), 4);
    }

}
