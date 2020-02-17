use tokio::net::TcpListener;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use std::error::Error;

pub async fn tcp_api(func: fn(&str) -> String, address: String) -> Result<!, Box<dyn Error>> {

    let mut listener = TcpListener::bind(address).await?;


    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {
                tokio::spawn(async move {
                    let (reader, mut writer) = socket.split();
                    let mut buf: Vec<u8> = vec![];
                    let mut reader: BufReader<_> = BufReader::new(reader);

                    loop {
                        let n: usize = reader.read_until(b';', &mut buf).await.unwrap();

                        let input = std::str::from_utf8(&buf[..n]).expect("Not valid utf-8");

                        let result = func(input);
                        eprintln!("{}", result);

                        buf.drain(..n);
                        writer.write_all(result.as_bytes()).await.unwrap();
                        writer.flush();
                    }
                });
            }
            Err(e) => println!("error accepting socket; error = {:?}", e),
        }
    }
}
