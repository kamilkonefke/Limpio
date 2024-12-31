// Networking
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use colored::Colorize;

// File operations
use std::fs;

// Configs
use crate::config::Config;
mod config;

macro_rules! VERBOSE {
    ($config:expr, $msg:expr) => {
        if $config.app.verbose {
            println!("{}", $msg);
        }
    }
}

fn main() {
    // Initialize cfg
    let config: Config = Config::load_from_file("docs/config.toml");
    match config.validate() {
        Ok(c) => run_server(&c),
        Err(e) => eprintln!("{}", format!("Error: {}", e).truecolor(255, 0, 0)),
    }
}

fn run_server(config: &Config) {
    println!("Listening on {}:{}", config.server.ip, config.server.port);
    let socket: String = format!("{}:{}", config.server.ip, config.server.port);
    let listener = TcpListener::bind(socket).expect("ERR");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, &config),
            Err(e) => println!("{}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream, config: &Config) {
    VERBOSE!(config, format!("CONNECTED: {:?}", stream.peer_addr().unwrap()));

    let buf_reader = BufReader::new(&stream);
    let upcoming_request = &buf_reader.lines().next().unwrap().unwrap();

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
        "Unknown error"
    };
    VERBOSE!(config, format!("REQUESTED FILE: {}", requsted_file));

    // Make request
    let requested_path = format!("{}/{}", config.server.root, requsted_file);

    // Check if file is avilable
    let (status, file) = if fs::exists(&requested_path).unwrap() == true {
        ("HTTP/1.0 200 OK", requested_path)
    } 
    else {
        ("HTTP/1.1 404 NOT FOUND", format!("{}/{}", config.server.root, "404.html"))
    };

    // Read file from filesystem
    let content = fs::read_to_string(file).unwrap();
    let content_length = content.len();

    let response = format!("{status}\r\nContent-Length: {content_length}\r\nContent-Type: text/html\r\n\r\n{content}");
    VERBOSE!(config, format!("RESPONSE: {}", response));

    stream.write_all(response.as_bytes()).unwrap(); // Send it to client
    stream.flush().unwrap();
}
