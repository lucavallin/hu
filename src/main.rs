mod settings;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use config::Config;
use hue_rs::apis::auth_api::authenticate;
use hue_rs::apis::configuration::Configuration;
use hue_rs::models::AuthenticateRequest;
use settings::Settings;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize hu, this will create a config file
    Init,
    /// Turns the light on
    On {
        /// The name of the light to turn on
        light: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let settings: Settings = Config::builder()
        .add_source(config::File::with_name("hu.toml"))
        .build()
        .context("Failed to load configuration")?
        .try_deserialize()
        .context("Failed to parse configuration")?;

    // Ensure the bridge IP is set
    if settings.bridge.ip.is_empty() {
        eprintln!("Error: 'bridge.ip' is not set in the configuration file");
        return Err(anyhow::anyhow!(
            "Error: 'bridge.ip' is not set in the configuration file"
        ));
    }

    match &cli.command {
        Some(Commands::Init) => initialize_hu(&settings).await,
        Some(Commands::On { light }) => turn_on_light(light),
        None => {
            Cli::command().print_help()?;
            Ok(())
        }
    }
}

/// Handles initialization logic
async fn initialize_hu(settings: &Settings) -> Result<()> {
    println!(
        "Initializing hu for bridge with IP: {}...",
        settings.bridge.ip
    );
    println!("Starting authentication...");
    println!("Please press the button on the bridge, then press Enter to continue.");

    let _ = std::io::stdin().read_line(&mut String::new());

    let mut configuration = Configuration::new();
    configuration.base_path = format!("http://{}", settings.bridge.ip);

    let authenticate_request = AuthenticateRequest {
        devicetype: Some(settings.auth.devicetype.to_string()),
        generateclientkey: Some(true),
    };

    let response = authenticate(&configuration, Some(authenticate_request))
        .await
        .context("Authentication failed")?;

    if let Some(success_response) = response.iter().find_map(|res| res.success.as_ref()) {
        println!(
            "Authentication successful! Response: {:?}",
            success_response
        );
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Authentication failed: No success field in response"
        ))
    }
}

/// Turns on the specified light
fn turn_on_light(light: &Option<String>) -> Result<()> {
    match light {
        Some(light_name) => {
            println!("Turning on light: {}", light_name);
        }
        None => {
            println!("Turning on all lights");
        }
    }
    Ok(())
}
