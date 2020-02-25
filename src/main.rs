#![feature(never_type)]

use algebraicdb::create_tcp_server;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<!, Box<dyn Error>> {
    #[cfg(features = "wrapper")]
    unimplemented!();

    create_tcp_server("127.0.0.1:5432").await
}
