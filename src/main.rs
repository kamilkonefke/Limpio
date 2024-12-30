use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Ipv4Addr;
use std::fs;

use serde::Deserialize;
use toml;

use colored::Colorize;

// Systemctl integration
#[derive(Debug, Deserialize)]
struct Config {
    server: Server,
}

#[derive(Debug, Deserialize)]
struct Server {
    ip: String,
    port: String,
    root: String,
    index: String,
    not_found: String,
}

fn main() {
    let config_file = fs::read_to_string("docs/config.toml").expect("Config not found");
    let config: Config = toml::de::from_str(&config_file).expect("Unknown config formatting");

    match check_config(&config) {
        Ok(_) => run_server(&config),
        Err(err) => eprintln!("{}", format!("Error: {}", err).truecolor(255, 0, 0)),
    }
}

fn run_server(config: &Config) {
    println!("Listening on {}:{}", config.server.ip, config.server.port);
    let socket: String = format!("{}:{}", config.server.ip, config.server.port);
    let listener = TcpListener::bind(socket).expect("ERR");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream, &config);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, config: &Config) {
    let buf_reader = BufReader::new(&stream);
    let upcoming_request = buf_reader.lines().next().unwrap().unwrap();

    // Get file path from request
    let requsted_file = if let Some(path) = upcoming_request.split_whitespace().nth(1) {
        if path == "/" {
            config.server.index.as_str()
        } 
        else {
            &path[1..]
        }
    }
    else {
        ""
    };

    // Make request
    let requested_path = format!("{}/{}", config.server.root, requsted_file);
    let (status, file) = if check_path(&requested_path) == true {
        ("HTTP/1.0 200 OK", requested_path)
    } 
    else {
        ("HTTP/1.1 404 NOT FOUND", format!("{}/{}", config.server.root, config.server.not_found))
    };

    let content = fs::read_to_string(file).unwrap();
    let content_length = content.len();

    let response = format!("{status}\r\nContent-Length: {content_length}\r\nContent-Type: text/html\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap(); // Send it
    stream.flush().unwrap();
}

fn check_config(config: &Config) -> Result<(), String> {
    if check_ip(config.server.ip.as_str()) == false {
        return Err("Check `ip` value".to_string());
    }
    if check_port(config.server.port.as_str()) == false {
        return Err("Check `port` value".to_string());
    }
    let relative_path = format!("{}/{}", config.server.root, config.server.index);
    if check_path(&relative_path) == false {
        return Err(format!("Error: Check `root` and `index` values. Path {} not found", &relative_path));
    }
    Ok(())
}

fn check_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}

fn check_port(port: &str) -> bool {
    match port.parse::<u16>() {
        Ok(v) => {
            if v > 0 && v <= u16::MAX {
                return true;
            }
            return false;
        },
        Err(_) => return false,
    }
}

fn check_path(path: &str) -> bool {
    fs::exists(path).unwrap()
}
