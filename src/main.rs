use std::io::{BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

use serde::Deserialize;
use toml;

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
}

fn main() {
    let config_file = fs::read_to_string("docs/config.toml").expect("Config not found");
    let config: Config = toml::de::from_str(&config_file).expect("ERR");

    check_config(&config);

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

    if upcoming_request == "GET / HTTP/1.1" {
        let status = "HTTP/1.1 200 OK";
        let content_path = format!("{}/{}", config.server.root, config.server.index);
        let content = fs::read_to_string(content_path).unwrap();
        let content_length = content.len();

        let response = format!("{status}\r\n{content_length}\r\n\r\n{content}");
        stream.write_all(response.as_bytes()).unwrap();
    }
    else {

    }
    // TODO: Handle rest of html requests and errors
}

fn check_config(config: &Config) {
    // TOOD: Check validation of ip, port, paths etc...
}
