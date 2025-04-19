use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream, TcpListener};

struct User {
    user_name: String,
}

struct Room {
    room_name: String,
    room_password: String,
    room_port: SocketAddr,
    user_count: usize,
    users: Vec<User>,
    max_user_count: usize,
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

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    try_connect();
    Ok(())
}

fn initialize() {
    println!("lcraft");
    println!("Type '/h' for help.");

    handle_input();
}

fn handle_input() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if !input.is_empty() {
            match input {
                "/h" | "/help" => show_help(),
                // "/join" => join_room(),
                "/host" => host_room(),
                // "/list" => list_rooms(),
                _ => println!("{}", input),
            }
        }
    }
}

fn show_help() {
    println!("Help Menu:");
    println!("just write it correctly");
}

fn host_room() {
    print!("Enter port you wish to host on: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Set room port
    match TcpListener::bind(input) {
        Ok(_) => {
            print!("Room created!\nCreate a name for the room: ");

        }
        Err(_) => {
            print!("Port already occupied.")
        }


    }
}

fn check_avail_connections(start_port: u16, end_port: u16) -> Vec<SocketAddr> {
    let mut available_ports = Vec::<SocketAddr>::new();

    // check connections through specified ports
    for port in start_port..end_port {
        let curr_socket_addr = SocketAddr::from(([127, 0, 0, 1], port));

        match TcpListener::bind(curr_socket_addr) {
            Ok(_) => {
                println!("Port {} is available.", port);
                available_ports.push(curr_socket_addr);
            }
            Err(_) => {}
        }
    }

    available_ports
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
