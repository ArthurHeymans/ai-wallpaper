mod config;
mod diffusion;
mod llm;

use clap::{Parser, Subcommand};
use config::AppConfig;
use dotenv::dotenv;
use log::{debug, info};
use std::env;
use std::path::Path;

#[derive(Parser)]
#[command(name = "wallpaper-generator")]
#[command(about = "Generates wallpapers using AI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to config file (default: ~/.config/wallpaper-generator/config.yaml)
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// Enable debug logging
    #[arg(long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate wallpaper using keywords
    Generate {
        /// Keywords to use for generating the wallpaper (uses defaults if not provided)
        #[arg(num_args = 0..)]
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

    // Initialize logger based on debug flag
    let log_level = if cli.debug {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::Builder::new().filter_level(log_level).init();

    // Determine config path
    let config_path = if let Some(path) = cli.config {
        path
    } else {
        // Use default config location
        let home = env::var("HOME").expect("HOME environment variable not set");
        let default_path = format!("{}/.config/wallpaper-generator/config.yaml", home);
        if Path::new(&default_path).exists() {
            default_path
        } else {
            println!("Config file not found, using template...");
            "templates/config.yaml.template".to_string()
        }
    };

    let config = AppConfig::from_file(&config_path)?;

    match cli.command {
        Commands::Generate { keywords, output } => {
            // Combine command-line keywords with default keywords
            let all_keywords: Vec<String> = keywords
                .iter()
                .chain(&config.default_keywords)
                .map(|s| s.to_string())
                .collect();

            let prompt = llm::generate_prompt(&config.llm_api, &all_keywords)
                .map_err(|e| format!("Failed to generate prompt: {}", e))?;
            debug!("Generated prompt: {}", prompt);

            let diffusion_client = diffusion::DiffusionClient::new(&config.diffusion.api);
            let wallpaper_url = diffusion_client
                .generate_wallpaper(
                    &prompt,
                    &config.diffusion.aspect_ratio,
                    config.diffusion.megapixels,
                )
                .map_err(|e| format!("Failed to generate wallpaper: {}", e))?;
            info!("Wallpaper generated, downloading to {}...", output);

            diffusion_client
                .download_image(&wallpaper_url, &output)
                .map_err(|e| format!("Failed to download wallpaper: {}", e))?;
            info!("Wallpaper saved to {}", output);
        }
        Commands::ListKeywords => {
            println!("Default keywords:"); // Keep this non-debug since it's a command output
            for keyword in &config.default_keywords {
                println!("- {}", keyword); // Keep this non-debug since it's a command output
            }
        }
    }

    Ok(())
}
