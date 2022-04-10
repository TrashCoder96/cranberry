use std::net::TcpListener;

#[tokio::main]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8181").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}
