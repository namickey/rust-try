use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established.");
        thread::spawn(|| {handle_connection(stream)});
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 2048];
    let i = stream.read(&mut buffer).unwrap();
    println!("usize: {i}");
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let contents = "hello world";
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    let h = stream.write(response.as_bytes()).unwrap();
    println!("usize: {h}");
    stream.flush().unwrap();
}
