

extern crate web_server;
use web_server::ThreadPool;
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs::File;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
    let stream = stream.unwrap();
    pool.execute(|| {
    handle_connection(stream);
    });
    }
    println!("Shutting down.");
    }
        
    

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    
    let get = b"GET / HTTP/1.1\r\n";

    let (statusline,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n ", "index.html")}
        else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{} {} ",statusline, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("request from {}", String::from_utf8_lossy(&buffer[..]));
    

}