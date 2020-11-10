use smol::Timer;
use std::time::Duration;
use std::net::{TcpListener, TcpStream};
use futures::io::{AsyncWriteExt};
use async_dup::Arc;
use smol::{io, Async};
use async_std::io::ReadExt;
use std::str;

async fn sleep(dur: Duration) {
    Timer::after(dur).await;
}

async fn read_ping(mut stream: Async<TcpStream>) -> io::Result<()> {
    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer).await?;
    let buffer = str::from_utf8(&buffer).unwrap();
    println!("{}", buffer);
    sleep(Duration::from_secs(1)).await;
    smol::spawn(write_pong(stream)).detach();
    Ok(())
}

async fn write_pong(stream: Async<TcpStream>) -> io::Result<()> {
    let mut writer = Arc::new(stream);
    writer.write_all(b"Pong").await?;
    //println!("Wrote Pong to stream");
    Ok(())
}

fn main() -> io::Result<()> {
    smol::block_on(async {
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 7000))?;
        println!("Listening on {}", listener.get_ref().local_addr()?);
        println!("Now start a TCP client."); 

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            println!("Accepted client: {}", peer_addr);
            smol::spawn(read_ping(stream)).detach();
        }
    })
}
