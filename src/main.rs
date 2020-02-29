#![feature(never_type)]

use algebraicdb::create_tcp_server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<!, Box<dyn Error>> {
    create_tcp_server("127.0.0.1:5001").await
}
