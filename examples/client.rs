use std::time::Duration;

use alluxio_rs::{client, option};
use tokio::io::BufReader;

#[tokio::main]
async fn main() {
    let path = "/test_path";
    let c = client::Client::new("localhost".to_string(), 39999, Duration::from_secs(10)).unwrap();

    let r = c.exists(path, option::Exists).await.unwrap();
    let s = "123";
    let d = s.as_bytes();
    let buf = BufReader::new(d);
    let id = c
        .create_file(path, option::CreateFile::default())
        .await
        .unwrap();
    c.write(id, BufReader::new(buf)).await.unwrap();
    c.close(id).await.unwrap();
    println!("{} {}", r, id);
}
