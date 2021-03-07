use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;
use webserver::Server::ThreadPool;
use webserver::RequestParser::get_request_info;
use webserver::MarkdownParser::parse_markdown;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut mode: &str = "HTML";
    stream.read(&mut buffer).unwrap();

    let get = b"GET";    

    let (status_line, filename) = if buffer.starts_with(get) {
        get_request_info(from_utf8(&buffer).unwrap())
    } else {
        ("HTTP/1.1 405 METHOD NOT ALLOWED \r\n\r\n".to_string(), "public/405.html".to_string())
    };

    if filename.contains(".md") {
        mode = "MD";
    }

    let contents = match fs::read_to_string(filename) {
        Ok(file_data) => file_data,
        Err(e) => fs::read_to_string("public/404.html").unwrap(),
    };

    let response_data = if mode == "MD" {
        parse_markdown(&contents)
    } else {
        contents
    }.to_string();

    let response = format!("{}{}", status_line, response_data);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}