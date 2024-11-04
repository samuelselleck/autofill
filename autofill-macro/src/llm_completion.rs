use crate::{macro_input::MacroOptions, AUTOFILL_DEBUG};

pub fn generate_completion(input: &str, options: &MacroOptions) -> Result<String, String> {
    if AUTOFILL_DEBUG {
        eprintln!("REQUESTING FROM LLM: {:#?},\n{:?}", options, input);
    }
    let api_key = env!(
        "ANTHROPIC_API_KEY",
        "ANTHROPIC_API_KEY must be set at compile time"
    );

    let prompt = options.prompt.as_ref().map(String::as_str).unwrap_or(
        r##"
You are completing Rust macro code. You should only output the exact code needed to make the provided code compile and run as intended, following these rules:

1. PATHS & IMPORTS:
- Fully qualify ALL paths with complete paths:
  - std: `std::vec::Vec`, `std::string::String`, `std::collections::HashMap`
  - external: `tokio::sync::Mutex`, `serde::Serialize`
  - Never use 'use' statements
  - Never use naked/unqualified types, even from prelude

2. NAMING:
- Prefix ALL helper items with `__autofill_`:
  - Functions: `__autofill_my_helper_fn`
  - Types: `__AutoFill_MyType`
  - Constants: `__AUTOFILL_CONSTANT`
  - Traits: `__AutoFill_MyTrait`

3. DEPENDENCIES:
- Only use types/functions from:
  - The standard library
  - Dependencies that are VERY OBVIOUSLY used in the provided code
- Never assume additional dependencies are available

4. OUTPUT FORMAT:
- Output ONLY the completed code
- No explanations, comments or surrounding text
- No "```rust" code blocks

The code to complete follows:
CODE:
"##,
    );

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 4096,
            "messages": [{
                "role": "user",
                "content": format!("{}\n\n```rust\n{}\n```", prompt, input)
            }]
        }))
        .send()
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "API request failed with status {}: {}",
            response.status(),
            response.text().unwrap_or_default()
        ));
    }

    let response_data: serde_json::Value = response
        .json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let content = response_data
        .get("content")
        .and_then(|content| content[0].get("text"))
        .and_then(|text| text.as_str())
        .ok_or("Failed to extract content from response")?;

    let code = if let Some(start) = content.find("```rust") {
        if let Some(end) = content[start..].rfind("```") {
            let code_block = &content[start..][7..end];
            code_block
                .lines()
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            content.to_string()
        }
    } else {
        content.to_string()
    };

    if AUTOFILL_DEBUG {
        eprintln!("Got response from LLM: {:?}", code);
    }
    Ok(code)
}
