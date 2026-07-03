

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let _ = handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>>{
    let buf_reader = BufReader::new(&mut stream);
    let mut lines = buf_reader.lines();
    
    let request_line = lines.next()
        .ok_or("No request line")?
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    
    let http_request: Vec<_> = lines
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let client_addr = stream.peer_addr()?;
    println!("Request: {request_line} - {client_addr:#?}\r\n{http_request:#?}");
    let path = request_line.split_whitespace().nth(1).ok_or("No path found")?;
    let path_without_query = path.split('?').next().ok_or("No path")?;
    let filename = if path_without_query == "/" {
        std::path::PathBuf::from("index.html")
    } else {
        let relative_path = path_without_query.trim_start_matches('/');
        if relative_path.contains("..") || relative_path.contains('*') {
            std::path::PathBuf::from("permission_denied.html")
        }
        else {
        std::path::PathBuf::from(relative_path)
        }
    };
    let (status_line, contents) = match fs::read(&filename) {
        Ok(contents) => ("HTTP/1.1 200 OK", contents),
        Err(_) => ("HTTP/1.1 404 NOT FOUND", fs::read("404.html").unwrap_or_else(|_| "404 Not Found".to_string().into())),
    };
    let length = contents.len();
    println!("Response line: {status_line}");
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
    Ok(())
}
