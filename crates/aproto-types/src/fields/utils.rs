use syn::parse::ParseStream;

use crate::fields::scalar::ScalarField;

use super::Label;

pub fn parse_label(input: ParseStream) -> syn::Result<Option<Label>> {
    let fork = input.fork();
    if fork.peek(syn::Ident) {
        let ident = fork.parse::<syn::Ident>().unwrap();
        let label = Label::from_str(ident.to_string().as_str());
        if label.is_some() {
            input.parse::<syn::Ident>()?;
        }
        return Ok(label);
    }
    Ok(None)
}

pub fn is_protobuf_reserve_key_word(word: &str) -> bool {

    if ScalarField::is_scalar_field(word) {
        return true;
    }

     // Check against other reserved words
     matches!(
        word,
        // Field labels
        "repeated" | "optional" | "required" |
        // Other protobuf keywords
        "message" | "enum" | "service" | "rpc" | "extend" |
        "extensions" | "option" | "package" | "import" |
        "public" | "weak" | "oneof" | "map" | "reserved" |
        "syntax" | "to" | "max" | "stream"
    )
}

#[allow(unused)]
pub fn is_rust_reserve_key_word(word: &str) -> bool {
    matches!(
        word,
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" |
        "extern" | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" |
        "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref" | "return" |
        "self" | "Self" | "static" | "struct" | "super" | "trait" | "true" |
        "type" | "unsafe" | "use" | "where" | "while" | "async" | "await" |
        "dyn" | "abstract" | "become" | "box" | "do" | "final" | "macro" |
        "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield" |
        "try" | "union"
    )
}
