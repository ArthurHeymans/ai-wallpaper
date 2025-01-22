use crate::config::ApiConfig;
use log::debug;
use reqwest::blocking::Client;
use serde_json::json;

pub fn generate_prompt(
    api_config: &ApiConfig,
    keywords: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let prompt_request = json!({
        "model": api_config.model,
        "messages": [{
            "role": "user",
            "content": format!("Create a short 100 word max prompt for generating a wallpaper using these keywords: {}", keywords.join(", "))
        }]
    });

    let response = client
        .post(format!("{}/v1/chat/completions", api_config.url))
        .header("Authorization", format!("Bearer {}", api_config.api_key))
        .json(&prompt_request)
        .send()?;

    let response_json: serde_json::Value = response.json()?;

    // Log the full response for debugging
    debug!("LLM API Response: {}", response_json);

    // Handle the response structure more carefully
    let prompt = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| {
            format!(
                "Failed to parse LLM response. Expected structure: choices[0].message.content\nFull response: {}",
                response_json
            )
        })?
        .to_string();

    Ok(prompt)
}
