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
You are completing Rust code in a macro-context. You should only output the exact code needed to make the provided code compile and run as intended, following these rules:


1. NAMING:
- Prefix ALL helper items with `__autofill_`:
  - Functions: `__autofill_my_helper_fn`
  - Types: `__AutoFill_MyType`
  - Constants: `__AUTOFILL_CONSTANT`
  - Traits: `__AutoFill_MyTrait`

2. DEPENDENCIES:
- Only use types/functions from:
  - The standard library
  - Dependencies that are VERY OBVIOUSLY used in the provided code
- Never assume additional dependencies are available

3. OUTPUT FORMAT:
- What you are witing is REPLACING the code bellow - remember to include everything needed to make it run.
- Output ONLY valid completed code of the functions/structs in the outlined code below.
- No explanations, comments or surrounding text
- No "```rust" code blocks
- Do NOT leave any code to be filled in later, complete everything.
- Include all the functions/structs in the input, even if already completed.
- Add whatever #[derive(..)]s you need to existing structs.

4. PATHS & IMPORTS:
- Remember that what you write will be PART of a file, and imports WILL conflict with
  other imports. 
- If for example HashMap is used in the code below, assume it's already imported.
- Fully qualify all paths with complete paths:
  - std: `std::vec::Vec`, `std::string::String`, `std::collections::HashMap`
  - external: `tokio::sync::Mutex`, `serde::Serialize`
- NEVER import using 'use' statements, instead fully qualify at each usage site.

The code to complete/fill-in follows:
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
            "temperature": 0.0,
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
