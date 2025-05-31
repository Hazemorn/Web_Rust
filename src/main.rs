#![warn(clippy::all, clippy::pedantic)]

use std::net::TcpListener;

use Web_server_Rust::ThreadPool;

mod handler;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
           handler::handle_connection(stream);
        });
    }
}