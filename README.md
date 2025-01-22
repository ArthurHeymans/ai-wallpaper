# AI Wallpaper Generator

A command-line tool that generates stunning wallpapers using AI models. Combines language models for prompt generation with diffusion models for image creation.

## Features

- 🎨 Generate wallpapers from text prompts
- 🤖 Automatic prompt generation using AI
- 🖼️ Customizable aspect ratio and resolution
- 🔑 Supports multiple AI APIs
- 📝 Configurable default keywords
- 🐚 Easy-to-use CLI interface
- 🐛 Debug logging support

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/ai-wallpaper.git
   cd ai-wallpaper
   ```

2. Install dependencies:
   ```bash
   cargo build --release
   ```

3. (Optional) Create a `.env` file with your API keys if you don't want to set them in your shell environment:
   ```bash
   # .env file content example:
   OPENAI_API_KEY=your_openai_key_here
   REPLICATE_API_TOKEN=your_replicate_token_here
   ```
   Alternatively, you can set the API keys in your shell environment:
   ```bash
   export OPENAI_API_KEY=your_key_here
   export REPLICATE_API_TOKEN=your_token_here
   ```

4. Create a config file:
   ```bash
   cp assets/config.yaml.template config.yaml
   # Edit config.yaml with your preferred settings
   ```

## Usage

### Generate a wallpaper with specific keywords
```bash
# Generate a space-themed wallpaper with futuristic elements
cargo run -- generate "space" "futuristic" "minimalism" --output space.png
```

### Generate a wallpaper using default keywords
```bash
# Uses default keywords from config.yaml (nature, futuristic, cyberpunk, photorealistic)
cargo run -- generate --output default.png
```

### List default keywords
```bash
cargo run -- list-keywords
```

### Enable debug logging
```bash
RUST_LOG=debug cargo run -- generate "nature" --output nature.png --debug
```

## Configuration

Edit `config.yaml` to customize:

- **LLM API settings** (for prompt generation)
- **Diffusion API settings** (for image generation)
- Default keywords
- Aspect ratio and resolution

Example config.yaml:
```yaml
llm_api:
  url: "https://api.openai.com"  # Base URL without endpoint path
  model: "gpt-4o"
  api_key: "${OPENAI_API_KEY}"

diffusion:
  api:
    url: "https://api.replicate.com/v1/predictions"
    model: "black-forest-labs/flux-dev"
    api_key: "${REPLICATE_API_TOKEN}"
  aspect_ratio: "16:9"
  megapixels: 1.0

default_keywords:
  - "nature"
  - "futuristic"
  - "cyberpunk"
  - "photorealistic"
```

### Daily Wallpaper with Cron

You can set up a cron job to automatically generate and set a new wallpaper each day:

1. Create a script to generate and set the wallpaper:
   ```bash
   #!/bin/bash
   # ~/bin/daily-wallpaper.sh
   
   # Generate new wallpaper with date in filename
   DATE=$(date +%Y-%m-%d)
   OUTPUT_FILE=~/Pictures/wallpapers/daily_${DATE}.png
   cargo run -- generate --output $OUTPUT_FILE
   
   # Set wallpaper with hyprpaper
   hyprctl hyprpaper preload "$OUTPUT_FILE"
   hyprctl hyprpaper wallpaper ",$OUTPUT_FILE"
   ```

2. Make the script executable:
   ```bash
   chmod +x ~/bin/daily-wallpaper.sh
   ```

3. Add a cron job to run daily at 8:00 AM:
   ```bash
   crontab -e
   ```
   Add this line:
   ```bash
   0 8 * * * ~/bin/daily-wallpaper.sh
   ```

4. (Optional) Create the wallpapers directory:
   ```bash
   mkdir -p ~/Pictures/wallpapers
   ```

## Requirements

- Rust 1.65+
- API keys for your chosen AI services

### Supported Models

This tool is designed to work with any LLM or image generation model that provides a REST API. The configuration is flexible and supports:

#### Language Models (LLM)
- OpenAI GPT models (default)
- Any other LLM with a compatible API
- Customizable via the `llm_api` section in config.yaml

#### Image Generation Models
- Replicate diffusion models (default)
- Any other image generation model with a compatible API
- Customizable via the `diffusion` section in config.yaml

To use different models:
1. Update the API endpoints in config.yaml
2. Adjust the model names/versions
3. Set the appropriate API keys

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- OpenAI for the language model API
- Replicate for the diffusion model hosting
- Rust community for excellent tooling
