//! cargo run --example tcp-server
//! cargo run --example tcp-client

//1. Send a `Ping` message every 5 secs.

//1. Start a timer
//    1. When any new message is received, reset the timer to 0
//    2. Once the timer reaches 10 secs then drop the connection

//use smol::io::AsyncWriteExt;

//use std::io::BufReader;
use futures::AsyncWriteExt;
//use async_dup::Arc;
//use std::io::{Read, Write};
use std::net::TcpStream;
//use async_std::net::TcpStream;
use smol::{io, Async};
//use async_std::io::ReadExt;

fn main() -> io::Result<()> {
    smol::block_on(async {

        let mut stream = Async::<TcpStream>::connect(([127, 0, 0, 1], 7000)).await?;
        println!("Connected to {}", stream.get_ref().peer_addr()?);

        stream.write_all(b"Ping").await?;
        println!("Wrote Ping to stream");

        Ok(())
    })
}

