use color_eyre::eyre::Result;
use std::env;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    loop {
        let (mut socket, addr) = listener.accept().await?;

        debug!("Incomming connection from: {}", &addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                let in_msg = String::from_utf8(buf[0..n].to_vec()).expect("Unable to get buffer");
                debug!("Received message: {}", &in_msg);

                let out_msg = format!("acknowledge, message received v0.1.8: {}", in_msg);
                debug!("Acknowledge message v0.1.8: {}", &out_msg);

                let out_buf = out_msg.into_bytes();

                socket
                    .write_all(&out_buf)
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
