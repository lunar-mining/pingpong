//! First start a server:
//! cargo run --example tcp-server
//! cargo run --example tcp-client

//1. When `Ping` message is received, reply instantly with `Pong`
//
//1. Start a timer
//    1. When any new message is received, reset the timer to 0
//    2. Once the timer reaches 10 secs then drop the connection

use std::net::{TcpListener, TcpStream};
use futures::AsyncWriteExt;
use async_dup::Arc;
use smol::{io, Async};
use async_std::io::ReadExt;

async fn pong(mut stream: Async<TcpStream>) -> io::Result<()> {
    //stream.read(&mut [0; 128])?;
    //let mut buffer = vec![0; 128];
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).await?;
    println!("{}", buffer);
    //println!("{:?}", stream);
    //let mut stream = async_dup::Mutex::new(stream);
    let mut writer = Arc::new(stream);
    writer.write_all(b"Pong").await?;
    //println!("wrong pong");

    //let mut reader = writer.clone();
    //reader.read_to_string(&mut buffer).await?;
    //println!("{}", buffer);
    //println!("Wrote pong to stream");
    //io::copy(&stream, &mut &stream).await?;
    // writes to stream, copies to stream
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
            // if statement here
            smol::spawn(pong(stream)).detach();
        }
    })
}
