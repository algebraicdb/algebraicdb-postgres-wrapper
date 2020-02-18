use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

pub async fn tcp_api(func: fn(&str) -> String, address: String) -> Result<!, Box<dyn Error>> {
    let mut listener = TcpListener::bind(address).await?;

    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => {
                tokio::spawn(async move {
                    let (reader, mut writer) = socket.split();
                    let mut buf = vec![];
                    let mut rest = String::new();
                    let mut reader: BufReader<_> = BufReader::new(reader);

                    loop {
                        let n: usize = reader.read_until(b';', &mut buf).await.unwrap();

                        let input = std::str::from_utf8(&buf[..n]).expect("Not valid utf-8");

                        rest.push_str(input);
                        let (result, rest2) = conga(func, rest);
                        rest = rest2;

                        // TODO: fix for unicode
                        buf.drain(..n);
                        match result {
                            Some(ret) => {
                                writer.write_all(ret.as_bytes()).await.unwrap();
                                writer.flush().await.unwrap();
                            }
                            None => (),
                        }
                    }
                });
            }
            Err(e) => println!("error accepting socket; error = {:?}", e),
        }
    }
}

// CONGA FIX EVERYTHING
fn conga(func: fn(&str) -> String, stmt: String) -> (Option<String>, String) {
    let mut in_string = false;
    let mut result = vec![];
    let mut lasti = 0;
    let chars = stmt.chars().enumerate();

    for (i, ch) in chars {
        // TODO: Handle escape characters
        if ch == '"' {
            in_string = !in_string;
        }

        if ch == ';' && !in_string {
            let q = &stmt[lasti..=i];
            result.push(func(q));
            lasti = i + 1;
        }
    }

    let mut rest = String::new();
    let mut ret = None;

    if lasti != (stmt.len() - 1) {
        rest = String::from(&stmt[lasti..stmt.len()]);
    }

    if !result.is_empty() {
        ret = Some(result.join("\n"));
    }

    (ret, rest)
}

#[cfg(test)]
pub mod tests {

    use super::conga;

    #[test]
    pub fn test_conga() {
        let s1 = "SELECT dsdasd FROM dadasd".to_string();
        let s2 = "SELECT dasdas FROM dasdasd; INSERT dadasd into sdadad;".to_string();

        let (r1, rest1) = conga(always_success, s1.clone());

        assert_eq!(r1, None);
        assert_eq!(rest1, s1);

        let (r2, rest2) = conga(always_success, s2.clone());

        r2.unwrap();

        assert_eq!(rest2, "");

    }

    fn always_success(_: &str) -> String{
        "Success".to_string()
    }
}