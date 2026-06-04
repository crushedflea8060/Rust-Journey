

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let _ = handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>>{
    let buf_reader = BufReader::new(&stream);
    let mut lines = buf_reader.lines();
    let request_line = lines.next().unwrap().unwrap();
    
    let http_request: Vec<_> = lines
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let client_addr = stream.peer_addr()?;
    println!("Request: {request_line} - {client_addr:#?}\r\n{http_request:#?}");
    let path = request_line.split_whitespace().nth(1).ok_or("No path found")?;
    let path_without_query = path.split('?').next().ok_or("No path")?;
    let filename = if path_without_query == "/" {
        "index.html".to_string()
    } else {
        Path::new(path_without_query)
            .file_name()
            .ok_or("No filename")?
            .to_str()
            .ok_or("Invalid filename")?
            .to_string()
    };
    println!("{filename}");
    let (status_line, contents) = match fs::read_to_string(&filename) {
        Ok(contents) => ("HTTP/1.1 200 OK", contents),
        Err(_) => ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("404.html").unwrap_or_else(|_| "404 Not Found".to_string())),
    };
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    Ok(())
}
