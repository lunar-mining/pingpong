//! cargo run --bin tcp-server
//! cargo run --bin tcp-client

use async_dup::Arc;
use async_std::io::ReadExt;
use futures::AsyncWriteExt;
use smol::{future, io, Async, Executor, Timer};
use std::net::TcpStream;
use std::str;
use std::thread;
use std::time::Duration;

async fn sleep(dur: Duration) {
    Timer::after(dur).await;
}

async fn hello_loop() {
    loop {
        println!("Hello fren");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn pingpong(ex: Arc<Executor<'_>>)  -> io::Result<()> {
    // spawn hello loop in parallel
    ex.spawn(async {
        hello_loop().await;
    })
    .detach();

    // connect to server
    let mut stream = Async::<TcpStream>::connect(([127, 0, 0, 1], 7000)).await?;
    println!("Connected to {}", stream.get_ref().peer_addr()?);
    stream.write_all(b"Ping").await?;

    // read pong
    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer).await?;
    let buffer = str::from_utf8(&buffer).unwrap();
    println!("{}", buffer);

    Ok(())
}

fn main() -> io::Result<()> {
    let ex = Executor::new();
    let ex_clone = Arc::new(ex);
    let ex1 = ex_clone.clone();
    ex_clone.spawn(async {
        pingpong(ex1).await;
    })
    .detach();
    thread::spawn(move || future::block_on(ex_clone.run(future::pending::<()>())));
    thread::sleep(Duration::from_secs(5));
    Ok(())
}
