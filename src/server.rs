use std::io;
use std::io::{Read, Write};
use std::net;

pub static DEFAULT_ADDRESS: &str = "0.0.0.0:8888";

pub fn run(address: &str) -> io::Result<()> {
    let listener = net::TcpListener::bind(address).expect("error run");
    info!("Witnet server listening on {}", address);

    for stream in listener.incoming() {
        handle_connection(stream?).expect("Error handling connection")
    }

    Ok(())
}

fn handle_connection(mut stream: net::TcpStream) -> io::Result<()> {
    info!(
        "Incoming connection from: {}",
        stream
            .peer_addr()
            .map(|addr| addr.to_string())
            .unwrap_or("unknown".to_string())
    );
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}
