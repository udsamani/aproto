use syn::parse::ParseStream;

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
