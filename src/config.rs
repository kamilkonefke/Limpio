use lazy_static::lazy_static;
use std::sync::Mutex;
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

#[derive(Clone, Debug)]
pub struct Config {
    pub host: HostConfig,
    pub cli: AppConfig,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct HostConfig {
    pub ip: String,
    pub port: String,
    pub root: String,
    pub index: String,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub verbose: bool,
}

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

pub fn load_config(file_path: &str) {
    let cli = Cli::parse();

    // Read config file
    let config_raw = fs::read_to_string(file_path).unwrap();
    let mut host_config: HostConfig = toml::from_str(&config_raw).expect("Check your config for typos!");
    let cli_config = AppConfig {
        verbose: cli.verbose,
    };

    // Apply cli config if found
    cli.ip.map(|ip| host_config.ip = ip);
    cli.port.map(|port| host_config.port = port);
    cli.serve.map(|path| {
        let parts: Vec<&str> = path.split("/").collect();
        let file = parts.last().unwrap_or(&"index.html");
        let dir = &parts[..parts.len() - 1];

        host_config.root = dir.join("/");
        host_config.index = file.to_string();
    });

    let mut config_lock = CONFIG.lock().unwrap();
    *config_lock = Some(Config {
        host: host_config,
        cli: cli_config,
    });
}

pub fn get_config() -> Config {
    let config_lock = CONFIG.lock().unwrap();
    return config_lock.clone().unwrap();
}

pub fn validate_config() -> Result<(), String>{
    let config = get_config();
    let root = config.host.root;

    if check_ip(&config.host.ip) == false {
        return Err("Check `ip` value.".to_string());
    }

    if check_port(&config.host.port) == false {
        return Err("Check `port` value.".to_string());
    }

    let index = config.host.index;
    let index_path = format!("{}/{}", root, index);
    if check_path(&index_path) == false {
        return Err(format!("Check `root` and `index` values. Path {} not found", index_path));
    }

    Ok(())
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
