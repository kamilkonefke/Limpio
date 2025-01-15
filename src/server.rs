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

    let listener = TcpListener::bind(socket).unwrap();

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

    VERBOSE!(format!("REQUEST: {}", upcoming_request));

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
        ("HTTP/1.1 200 OK", requested_path)
    } 
    else {
        ("HTTP/1.1 404 NOT FOUND", format!("{}/{}", cfg.host.root, "404.html"))
    };

    // Read file from filesystem
    let content = fs::read(file.clone()).unwrap();
    let content_length = content.len();
    let content_type = get_content_type(&file);

    let response = format!("{status}\r\nContent-Length: {content_length}\r\nContent-Type: {content_type}\r\n\r\n");

    // Send response and content to client
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&content).unwrap();

    VERBOSE!(format!("RESPONSE:\n{}", response));

    stream.flush().unwrap();
}

fn get_content_type(file: &str) -> String {
    let extension = match file.split('.').last() {
        Some(ext) => ext,
        None => return "application/octet-stream".to_string(),
    };

    match extension.to_lowercase().as_str() {
        "html" | "htm" => "text/html".to_string(),
        "css" => "text/css".to_string(),
        "js"  => "application/javascript".to_string(),
        "json"=> "application/json".to_string(),
        "jpeg" | "jpg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        "bmp" => "image/bmp".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "txt" => "text/plain".to_string(),
        "pdf" => "application/pdf".to_string(),
        "zip" => "application/zip".to_string(),
        _ => "application/octet-stream".to_string(), // Default
    }
}
