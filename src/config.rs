use serde::Deserialize;
use std::net::Ipv4Addr;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "Limpio")]
#[command(version = "0.1")]
#[command(about = "Simple and easy to use webserver.", long_about = None)]
struct Cli {
    #[arg(short, long, action)]
    verbose: bool,
    /// Temporaily set the listening IP address for server.
    #[arg(short, long, value_name = "127.0.0.1")]
    ip: Option<String>,
    /// Temporaily set the listening port for server.
    #[arg(short, long, value_name = "8080")]
    port: Option<String>,
    /// Temporarily set the file path to be served by the server.
    #[arg(short, long, value_name = "docs/index.html")]
    serve: Option<String>,
}

pub struct Config {
    pub server: ServerConfig,
    pub app: AppConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: String,
    pub root: String,
    pub index: String,
}

pub struct AppConfig {
    pub verbose: bool,
}

impl Config {
    pub fn load_from_file(file_path: &str) -> Self {
        let cli = Cli::parse();

        // Read config file
        let config_raw = fs::read_to_string(file_path).expect("ERR");
        let mut server_config: ServerConfig = toml::de::from_str(&config_raw).expect("ERR");
        let app_config = AppConfig {
            verbose: cli.verbose,
        };

        // Apply cli config if found
        cli.ip.map(|ip| server_config.ip = ip);
        cli.port.map(|port| server_config.port = port);
        cli.serve.map(|path| {
            let parts: Vec<&str> = path.split("/").collect();
            let file = parts.last().unwrap_or(&"index.html");
            let dir = &parts[..parts.len() - 1];

            server_config.root = dir.join("/");
            server_config.index = file.to_string();
        });

        Config { 
            server: server_config,
            app: app_config,
        }
    }

    pub fn validate(self) -> Result<Self, String> {
        let root = &self.server.root;

        let ip = &self.server.ip;
        if check_ip(ip) == false {
            return Err("Check `ip` value.".to_string());
        }

        let port = &self.server.port;
        if check_port(port) == false {
            return Err("Check `port` value.".to_string());
        }

        let index = &self.server.index;
        let index_path = format!("{}/{}", root, index);
        if check_path(&index_path) == false {
            return Err(format!("Check `root` and `index` values. Path {} not found", index_path));
        }

        Ok(self)
    }
}

pub fn check_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}

pub fn check_port(port: &str) -> bool {
    match port.parse::<u16>() {
        Ok(p) => {
            if p > 0 && p <= u16::MAX {
                return true;
            }
            return false;
        },
        Err(_) => return false,
    }
}

pub fn check_path(path: &str) -> bool {
    fs::exists(path).unwrap()
}
