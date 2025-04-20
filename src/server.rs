use std::io::{Read, Write};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream, TcpListener};

use crate::libs::{User, Room};

#[derive(Default)]
pub struct Server {
    rooms: HashMap<String, Room>,
}

impl Server {
    fn new() -> Self {
        Server {
            rooms: HashMap::new(),
        }
    }

    pub fn start_server() -> std::io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:8080")?;
    
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New client connected!");
                    handle_client(stream);
                }
                Err(e) => {
                    eprintln!("Error accepting client: {}", e);
                }
            }
        }
        Ok(())
    }


    fn broadcast_message(&self, room_name: &str, message: &str) {
        if let Some(room) = self.rooms.get(room_name) {
            for user in room.get_users() {
                // send message
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(_) => {
            // idk
        }
        Err(e) => {
            eprintln!("Error reading from stream {}", e);
        }
    }
}


fn try_connect() {
    for n in 8000..9000 {
        let curr_socket_addr = SocketAddr::from(([127, 0, 0, 1], n));

        if let Ok(stream) = TcpStream::connect(curr_socket_addr) {
            println!("Connected to the server! {}", curr_socket_addr);
            break;
        } else {
            println!("Couldn't connect to server... {}", curr_socket_addr);
        }
    }
}
