use std::io::{Read, Write, Result};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{Arc, Mutex};

const MSG_SIZE: usize = 512;

lazy_static::lazy_static! {
    pub static ref GLOBAL_ROOMS: Arc<Mutex<Vec<Room>>> = Arc::new(Mutex::new(Vec::<Room>::new()));
}

use crate::libs::Room;

pub fn start_server() -> Result<()> {
    // create central listener
    let default_listener = TcpListener::bind("0.0.0.0:8080")?;

    for stream in default_listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Client connected");

                let rooms = Arc::clone(&GLOBAL_ROOMS);
                thread::spawn(move || {
                    handle_client(stream, rooms);
                });
            }
            Err(e) => {
                eprintln!("Server error: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, rooms: Arc<Mutex<Vec<Room>>>) {
    let mut buffer = [0; MSG_SIZE];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // disconnected
                println!("Disconnected");
                break;
            },
            Ok(size) => {
                let msg = String::from_utf8_lossy(&buffer[..size]).to_string();

                match msg.as_str() {
                    _ => {
                        stream.write_all("You can join a dedicated room with /join".as_bytes()).expect("Could not print msg");
                    }
                }
    
            },
            Err(e) => {
                eprintln!("Error reading request: {}", e);
            }
        }
    }
}