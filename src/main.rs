use colored::Colorize;

// Configs
mod config;
use config::*;

mod server;
use server::*;

fn main() {
    load_config("/etc/limpio/config.toml");
    
    match validate_config() {
        Ok(_) => run_server(),
        Err(e) => eprintln!("{}", format!("{}", e).truecolor(255, 0, 0)),
    }
}
