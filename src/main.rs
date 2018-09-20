#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate clap;

extern crate witnet_config as config;
extern crate witnet_core as core;
extern crate witnet_crypto as crypto;
extern crate witnet_data_structures as data_structures;
extern crate witnet_p2p as p2p;
extern crate witnet_storage as storage;

use std::process::exit;

mod cli;
mod server;

fn main() {
    env_logger::init();

    let matches = app_from_crate!()
        .subcommand(cli::server::get_arg())
        .get_matches();

    match matches.subcommand() {
        ("server", Some(arg_matches)) => {
            let address = arg_matches
                .value_of("address")
                .unwrap_or(server::DEFAULT_ADDRESS);

            server::run(address).expect("Server error");
        }
        _ => {
            println!("Unrecognized command. Run with '--help' to learn more.");
            exit(1);
        }
    }
}
