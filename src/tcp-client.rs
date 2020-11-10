//! cargo run --example tcp-server
//! cargo run --example tcp-client

//use smol::Timer;
//use std::time::Duration;
use std::net::TcpStream;
use futures::AsyncWriteExt;
use smol::{io, Async};
use async_std::io::ReadExt;
use std::str;

//async fn sleep(dur: Duration) {
//    Timer::after(dur).await;
//}
// TODO: start a smol task in parallel at start
// task loops and every 5 seconds prints hello

fn main() -> io::Result<()> {
    smol::block_on(async {

        let mut stream = Async::<TcpStream>::connect(([127, 0, 0, 1], 7000)).await?;
        println!("Connected to {}", stream.get_ref().peer_addr()?);

        stream.write_all(b"Ping").await?;
        //println!("Wrote Ping to stream");

        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).await?;
        let buffer = str::from_utf8(&buffer).unwrap();
        println!("{}", buffer);
        Ok(())
    })
}
