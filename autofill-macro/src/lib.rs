use macro_input::MacroInput;
use proc_macro::TokenStream;
use proc_macro2::Span;
use prompt_cache::AutofillCache;
use syn::parse_macro_input;

mod llm_completion;
mod macro_input;
mod prompt_cache;

#[cfg(test)]
mod test;

const AUTOFILL_DEBUG: bool = option_env!("AUTOFILL_DEBUG").is_some();

#[derive(thiserror::Error, Debug)]
enum AutofillError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[proc_macro]
pub fn autofill(input: TokenStream) -> TokenStream {
    let MacroInput { options, code } = parse_macro_input!(input as MacroInput);

    let code_str = code.to_string();

    // Initialize cache
    let cache = match AutofillCache::new() {
        Ok(cache) => cache,
        Err(e) => {
            return syn::Error::new(
                Span::call_site().into(),
                format!("Failed to initialize cache: {}", e),
            )
            .to_compile_error()
            .into();
        }
    };

    // Try to get from cache
    if let Some(cached) = cache.get_cached(&code_str, &options) {
        return cached.parse().unwrap_or_else(|_| code.into());
    }

    if !option_env!("AUTOFILL")
        .and_then(|a| a.parse::<bool>().ok())
        .unwrap_or_default()
    {
        return code.into();
    }

    // Cache miss - generate new content
    let new_content = match llm_completion::generate_completion(&code_str, &options) {
        Ok(content) => content,
        Err(e) => {
            return syn::Error::new(
                Span::call_site().into(),
                format!("Failed to generate completion: {}", e),
            )
            .to_compile_error()
            .into();
        }
    };

    // Store in cache
    if let Err(e) = cache.store(&code_str, options, new_content.clone()) {
        return syn::Error::new(
            Span::call_site().into(),
            format!("Failed to store in cache: {}", e),
        )
        .to_compile_error()
        .into();
    }

    new_content.parse().unwrap_or_else(|_| code.into())
}
