static TCP_ADDRESS: &str = "0.0.0.0:8888";

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;

extern crate witnet_config as config;
extern crate witnet_core as core;
extern crate witnet_crypto as crypto;
extern crate witnet_data_structures as data_structures;
extern crate witnet_p2p as p2p;
extern crate witnet_storage as storage;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write, Error};

use clap::{App, SubCommand };

fn main() {
    env_logger::init();

    let matches = App::new("rust witnet")
        .version("0.1.0")
        .author("Witnet Foundation <info@witnet.foundation>")
        .subcommand(SubCommand::with_name("server")
            .about("Running witnet protocol"))
        .get_matches();

    match matches.subcommand() {
        ("server", Some(_server_args)) => {
            run_sever();
            return;
            }
        _ => {
            warn!("No option specified");
            return;
        }
    }
}

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    info!("Incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 { return Ok(()) }
        stream.write(&buf[..bytes_read])?;
    }
}

fn run_sever () {
    let listener = TcpListener::bind(&TCP_ADDRESS).expect("Could not bind");
    info!("Listening on 0.0.0.0:8888");

    for stream in listener.incoming() {
        match stream {
            Err(e) => { error!("failed: {}", e) }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| error!("{:?}", error));
                });
            }
        }
    }
}
