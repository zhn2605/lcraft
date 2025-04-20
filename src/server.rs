
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, TcpListener};

struct User {
    user_name: String,
    curr_room: SocketAddr,
}

struct Room {
    room_name: String,
    room_password: String,
    room_port: SocketAddr,
    user_count: usize,
    users: Vec<User>,
    max_user_count: usize,
}

pub fn start_server() -> std::io::Result<()> {
    let mut rooms: Vec<Room> = Vec::<Room>::new();
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

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(_) => {
            let message = String::from_utf8_lossy(&buffer);
            println!("Recieved message: {}", message);

            let response = b"Message recieved";
            if let Err(e) = stream.write_all(response) {
                eprintln!("Failed to send response: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error reading from stream {}", e);
        }
    }
}

impl Room {
    fn new(room_name: String, room_port: SocketAddr, max_user_count: usize) -> Self {
        Self {
            room_name,
            room_password: String::new(),
            room_port,
            user_count: 0,
            users: Vec::new(),
            max_user_count,
        }
    }

    fn set_password(&mut self, pswd: &str) {
        if !pswd.is_empty() && pswd != self.room_password {
            self.room_password = pswd.to_string();
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
