use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "apibrowser")]
#[command(about = "Browse FRC Finger Lakes Regional data from The Blue Alliance")]
pub struct Cli {
   /// Use test mode with 2024 data instead of 2026
   #[arg(short, long)]
   pub test_mode: bool,

   /// Override event key (advanced usage)
   #[arg(short, long)]
   pub event_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
   pub api_key: String,
   pub event_key: String,
   pub test_mode: bool,
}

impl Config {
   pub fn load() -> Result<Self> {
      // Try loading .env from multiple locations:
      // 1. Current directory
      // 2. ~/.config/apibrowser/.env
      // 3. Environment variable directly

      // Try current directory first
      dotenvy::dotenv().ok();

      // If not found, try config directory
      if std::env::var("TBA_API_KEY").is_err() {
         if let Some(home) = dirs::home_dir() {
            let config_env = home.join(".config").join("apibrowser").join(".env");
            if config_env.exists() {
               dotenvy::from_path(&config_env).ok();
            }
         }
      }

      // Parse CLI arguments
      let cli = Cli::parse();

      // Get API key from environment
      let api_key = std::env::var("TBA_API_KEY")
         .unwrap_or_default()
         .trim()
         .to_string();

      // Determine event key first
      let event_key = if let Some(key) = cli.event_key {
         key
      } else if cli.test_mode {
         "2024nyro".to_string() // Use 2024 for test mode
      } else {
         "2026nyro".to_string()
      };

      // Check if API key is valid
      if api_key == "your_api_key_here" || api_key.is_empty() {
         anyhow::bail!(
            "TBA API requires authentication for all endpoints.\n\
                 Please set a valid TBA_API_KEY in your .env file.\n\
                 Get your API key from: https://www.thebluealliance.com/account"
         );
      }

      Ok(Config {
         api_key,
         event_key,
         test_mode: cli.test_mode,
      })
   }
}
