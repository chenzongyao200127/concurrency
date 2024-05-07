use anyhow::Result;
use tokio::io::{self, AsyncWriteExt};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // build a listener
    let addr = "0.0.0.0:9999";

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on: {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        tokio::spawn(async move {
            if let Err(e) = process_redis_connection(stream, raddr).await {
                warn!("Error processing connection: {:?}", e);
            }
        });
    }
}

async fn process_redis_connection(
    mut stream: tokio::net::TcpStream,
    raddr: std::net::SocketAddr,
) -> Result<()> {
    loop {
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("Read {} bytes", n);
                let lines = String::from_utf8_lossy(&buf);
                info!("Data: {:?}", lines);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection {} closed", raddr);
    Ok(())
}
