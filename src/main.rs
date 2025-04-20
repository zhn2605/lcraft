use std::env;
use std::process;

use server::Server;

mod client;
mod server;
mod libs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run [server|client]");
        process::exit(1);
    }

    match args[1].as_str() {
        "server" => Server::start_server().unwrap(),
        "client" => client::start_client(),
        _ => {
            eprintln!("Invalid argument. Use 'server' or 'client'.");
            process::exit(1);
        }
    }
}
