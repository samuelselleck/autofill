use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Result as SynResult, Token,
};

pub struct MacroInput {
    pub options: MacroOptions,
    pub code: proc_macro2::TokenStream,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // First parse the parenthesized options (if any)
        let options = if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            content.parse()?
        } else {
            MacroOptions::default()
        };

        // Parse the remaining tokens as the code
        let code = input.parse()?;

        Ok(MacroInput { options, code })
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Hash, Default, Debug)]
pub struct MacroOptions {
    pub prompt: Option<String>,
}

impl Parse for MacroOptions {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut prompt = None;

        if input.is_empty() {
            return Ok(MacroOptions { prompt: None });
        }

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;

            match ident.to_string().as_str() {
                "prompt" => {
                    let value: LitStr = input.parse()?;
                    prompt = Some(value.value());
                }
                key => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown option: {}", key),
                    ));
                }
            }

            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(MacroOptions { prompt })
    }
}
