// Networking
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

// File operations
use std::fs;

// Configs
use crate::config::*;

macro_rules! VERBOSE {
    ($msg:expr) => {
        if get_config().cli.verbose {
            println!("{}", $msg);
        }
    }
}

pub async fn run_server() {
    let cfg = get_config();
    println!("Listening on {}:{}", cfg.host.ip, cfg.host.port);
    let socket: String = format!("{}:{}", cfg.host.ip, cfg.host.port);
    let listener = TcpListener::bind(socket).expect("ERR");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream).await,
            Err(e) => println!("{}", e),
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let cfg = get_config();

    VERBOSE!(format!("CONNECTED: {:?}", stream.peer_addr().unwrap()));

    let buf_reader = BufReader::new(&stream);
    let upcoming_request = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(_)) => {
            return;
        },
        None => {
            return;         
        },
    };

    // Get file path from request
    let requsted_file = if let Some(path) = upcoming_request.split_whitespace().nth(1) {
        if path == "/" {
            cfg.host.index.as_str()
        } 
        else {
            &path[1..]
        }
    }
    else {
        ""
    };

    VERBOSE!(format!("REQUESTED FILE: {}", requsted_file));

    // Make path
    let requested_path = format!("{}/{}", cfg.host.root, requsted_file);

    // Check if file is avilable
    let (status, file) = if fs::exists(&requested_path).unwrap() == true {
        ("HTTP/1.0 200 OK", requested_path)
    } 
    else {
        ("HTTP/1.1 404 NOT FOUND", format!("{}/{}", cfg.host.root, "404.html"))
    };

    // Read file from filesystem
    let content = fs::read_to_string(file).unwrap();
    let content_length = content.len();

    let response = format!("{status}\r\nContent-Length: {content_length}\r\nContent-Type: text/html\r\n\r\n{content}");

    VERBOSE!(format!("RESPONSE: {}", response));

    stream.write_all(response.as_bytes()).unwrap(); // Send it to client
    stream.flush().unwrap();
}
