use crate::config::ApiConfig;
use log::debug;
use reqwest::blocking::Client;
use serde_json::json;

pub struct DiffusionClient {
    client: Client,
    api_url: String,
    api_key: String,
    model_version: String,
}

impl DiffusionClient {
    pub fn new(api_config: &ApiConfig) -> Self {
        Self {
            client: Client::new(),
            api_url: api_config.url.clone(),
            api_key: api_config.api_key.clone(),
            model_version: api_config.model.clone(),
        }
    }

    pub fn generate_wallpaper(
        &self,
        prompt: &str,
        aspect_ratio: &str,
        megapixels: f32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let inputs = json!({
            "prompt": prompt,
            "guidance": 3.5,
            "aspect_ratio": aspect_ratio,
            "megapixels": megapixels.to_string(),
            "output_format": "png"
        });

        // Create prediction
        let prediction = self
            .client
            .post(format!(
                "{}/v1/models/{}/predictions",
                self.api_url, self.model_version
            ))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "input": inputs
            }))
            .send()?;

        let mut prediction_json: serde_json::Value = prediction.json()?;
        debug!("Initial prediction response: {}", prediction_json);

        let prediction_id = prediction_json["id"]
            .as_str()
            .ok_or("Failed to get prediction ID")?
            .to_string();

        // Poll for completion with timeout
        let mut status = "starting".to_string();
        let mut output = None;
        let mut attempts = 0;
        const MAX_ATTEMPTS: u8 = 30; // 30 attempts * 2 seconds = 1 minute timeout

        while (status == "starting" || status == "processing") && attempts < MAX_ATTEMPTS {
            std::thread::sleep(std::time::Duration::from_secs(2));
            attempts += 1;

            let status_response = self
                .client
                .get(format!(
                    "https://api.replicate.com/v1/predictions/{}",
                    prediction_id
                ))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .send()?;

            prediction_json = status_response.json()?;
            debug!(
                "Prediction status (attempt {}): {}",
                attempts, prediction_json
            );

            status = prediction_json["status"]
                .as_str()
                .ok_or("Failed to get prediction status")?
                .to_string();

            if let Some(output_val) = prediction_json["output"].as_array() {
                if !output_val.is_empty() {
                    output = Some(output_val.clone());
                    break;
                }
            }
        }

        if attempts >= MAX_ATTEMPTS {
            return Err(format!("Prediction timed out after {} attempts", MAX_ATTEMPTS).into());
        }

        if status != "succeeded" {
            return Err(format!("Prediction failed with status: {}", status).into());
        }

        let image_url = match output {
            Some(v) => v
                .first()
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or("Failed to get output URL")?,
            None => return Err("No output received from model".into()),
        };

        Ok(image_url)
    }

    pub fn download_image(
        &self,
        url: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client.get(url).send()?;

        let mut file = std::fs::File::create(output_path)?;
        let mut content = std::io::Cursor::new(response.bytes()?);
        std::io::copy(&mut content, &mut file)?;

        Ok(())
    }
}
