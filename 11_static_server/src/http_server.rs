
use std::{
    io::{Read, Write},
    net::TcpListener,
    net::TcpStream,

};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connection: {}", addr);
        // thread::spawn(move || {
        //     let mut buf = [0u8; 12];
        //     stream.read_exact(&mut buf).unwrap();
        //     println!("data: {:?}", String::from_utf8_lossy(&buf));
        //     // 一共写了 17 个字节
        //     stream.write_all(b"glad to meet you!").unwrap();
        // });
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let content = "<font color='red'>Something is Response from Server</font>";
    let header = "Content-Type: text/html;charset=utf-8\r\nServer: nginx/1.8.0";
    let response = format!("{}\r\n{}\r\n{}", "HTTP/1.1 200 OK", header,content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}