mod config;
mod flux;
mod llm;

use clap::{Parser, Subcommand};
use config::AppConfig;
use dotenv::dotenv;
use std::path::Path;

#[derive(Parser)]
#[command(name = "wallpaper-generator")]
#[command(about = "Generates wallpapers using AI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate wallpaper using keywords
    Generate {
        keywords: Vec<String>,

        #[arg(short, long, default_value = "wallpaper.png")]
        output: String,
    },
    /// List default keywords from config
    ListKeywords,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cli = Cli::parse();

    // If config file doesn't exist, use the template
    let config_path = if Path::new(&cli.config).exists() {
        cli.config.clone()
    } else {
        println!("Config file not found, using template...");
        "templates/config.yaml.template".to_string()
    };

    let config = AppConfig::from_file(&config_path)?;

    match cli.command {
        Commands::Generate { keywords, output } => {
            let prompt = llm::generate_prompt(&config.llm_api, &keywords)
                .map_err(|e| format!("Failed to generate prompt: {}", e))?;
            println!("Generated prompt: {}", prompt);

            let flux_client = flux::FluxClient::new(&config.flux.api);
            let wallpaper_url = flux_client
                .generate_wallpaper(&prompt, &config.flux.aspect_ratio, config.flux.megapixels)
                .map_err(|e| format!("Failed to generate wallpaper: {}", e))?;
            println!("Wallpaper generated, downloading to {}...", output);

            flux_client
                .download_image(&wallpaper_url, &output)
                .map_err(|e| format!("Failed to download wallpaper: {}", e))?;
            println!("Wallpaper saved to {}", output);
        }
        Commands::ListKeywords => {
            println!("Default keywords:");
            for keyword in &config.default_keywords {
                println!("- {}", keyword);
            }
        }
    }

    Ok(())
}
