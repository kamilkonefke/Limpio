use colored::Colorize;

// Configs
mod config;
use config::*;

mod server;
use server::*;

#[tokio::main]
async fn main() {
    let config_path = "docs/config.toml".to_string();
    load_config(&config_path);
    
    match validate_config() {
        Ok(_) => run_server().await,
        Err(e) => eprintln!("{}", format!("{} {}", e, config_path).red()),
    }
}
