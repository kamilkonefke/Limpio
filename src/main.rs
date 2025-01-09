use colored::Colorize;

// Configs
mod config;
use config::*;

mod server;
use server::*;

#[tokio::main]
async fn main() {
    load_config("docs/config.toml");
    
    match validate_config() {
        Ok(_) => run_server().await,
        Err(e) => eprintln!("{}", format!("{}", e).truecolor(255, 0, 0)),
    }
}
