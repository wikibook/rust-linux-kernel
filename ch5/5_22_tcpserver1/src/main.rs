use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut len_buffer = [0u8; 8];
    stream.read_exact(&mut len_buffer).unwrap();
    let recv_len = i64::from_ne_bytes(len_buffer).try_into().unwrap();

    let mut txt_buffer = vec![0u8; recv_len];
    stream.read_exact(&mut txt_buffer).unwrap();
    
    let str = String::from_utf8(txt_buffer.to_vec()).unwrap();
    println!("클라이언트: {:?}", str);

    let hello = String::from("안녕! 서버!");
    let bytes = hello.as_bytes();
    let len = bytes.len();
    
    stream.write_all(&len.to_ne_bytes()).unwrap();
    stream.write_all(&bytes);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream);
    }
}
