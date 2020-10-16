use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
  let mut guess = String::with_capacity(10);
  io::stdin().read_line(&mut guess).expect("Failed to read");
  println!("Your guess: {}", guess);
}

fn start_tcp_demo() {
  println!("Started server.");
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  for stream in listener.incoming() {
    let stream = stream.unwrap();
    handle_stream(stream);
  }
}

fn handle_stream(mut stream: TcpStream) {
  let mut buffer = [0; 1024];
  stream.read(&mut buffer).unwrap();
  println!(
    "***TCP STREAM RECEIVED***\n{}\n***",
    String::from_utf8_lossy(&buffer[..])
  );
  let response = "HTTP/1.1 200 OK\r\n\r\n";
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}
