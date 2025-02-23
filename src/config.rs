use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct ApiConfig {
    pub url: String,
    pub model: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct DiffusionConfig {
    pub api: ApiConfig,
    pub aspect_ratio: String,
    pub megapixels: f32,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub llm_api: ApiConfig,
    pub diffusion: DiffusionConfig,
    pub default_keywords: Vec<String>,
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(Path::new(path))?;
        let config: AppConfig = serde_yaml::from_str(&config_str)?;

        // Substitute environment variables in the config
        Ok(AppConfig {
            llm_api: ApiConfig {
                url: config.llm_api.url,
                model: config.llm_api.model,
                api_key: Self::substitute_env_var(&config.llm_api.api_key)?,
            },
            diffusion: DiffusionConfig {
                api: ApiConfig {
                    url: config.diffusion.api.url,
                    model: config.diffusion.api.model,
                    api_key: Self::substitute_env_var(&config.diffusion.api.api_key)?,
                },
                aspect_ratio: config.diffusion.aspect_ratio,
                megapixels: {
                    // Validate megapixels is either 1.0 or 0.25
                    if config.diffusion.megapixels != 1.0 && config.diffusion.megapixels != 0.25 {
                        return Err(format!(
                            "Invalid megapixels value: {}. Must be either 1.0 or 0.25",
                            config.diffusion.megapixels
                        )
                        .into());
                    }
                    config.diffusion.megapixels
                },
            },
            default_keywords: config.default_keywords,
        })
    }

    fn substitute_env_var(value: &str) -> Result<String, Box<dyn std::error::Error>> {
        if value.starts_with("${") && value.ends_with('}') {
            let var_name = &value[2..value.len() - 1];
            env::var(var_name)
                .map_err(|_| format!("Environment variable {} not found", var_name).into())
        } else {
            Ok(value.to_string())
        }
    }
}
