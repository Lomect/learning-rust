use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;

struct ThreadPool;

impl ThreadPool {
    fn new(size: u32) -> ThreadPool { ThreadPool }
    fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8090").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();;
        println!("Tcp Stream!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let content = fs::read_to_string("hello.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        println!("Other!");
    }
}
