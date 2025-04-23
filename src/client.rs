use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, Shutdown, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use crate::libs::{User, Room};
use crate::server::GLOBAL_ROOMS;

const MSG_SIZE: usize = 512;
const ATTEMPT_CONNECT_TIME: Duration = Duration::from_millis(5000);

// Client entry point
pub fn start_client() {
    println!("Client started.");

    let user = initialize_user();
    println!("Welcome {}. Type '/h' to view all commands.", user.user_name);
    handle_input(&user);
    println!("Goodbye!");
}

fn initialize_user() -> User {
    print!("Enter user name:\n> ");
    io::stdout().flush().unwrap();

    let user_name = loop {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if !input.is_empty() && input.chars().all(|c| c.is_alphanumeric()) {
            break input.to_string();
        } else {
            println!("Must be alphanumeric.\nEnter user name:\n> ");
            io::stdout().flush().unwrap();
        }
    };

    User {
        user_name: user_name,
    }
}

fn handle_input(user: &User) {
    let mut stream: Option<TcpStream> = None;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if !input.is_empty() {
            // parse commands
            let parts: Vec<&str> = input.split_whitespace().collect();
            match parts[0] {
                "/h" | "/help" => show_help(),
                "/list" => {
                    let mut start_port: u16 = 8000;
                    let mut end_port: u16 = 9000;

                    if parts.len() > 3 {
                        end_port = parts[3].parse().unwrap();
                    }
                    if parts.len() > 2 {
                        start_port = parts[2].parse().unwrap()
                    }
                    if parts.len() > 1 {
                        if parts[1].to_lowercase() == "rooms" {
                            list_rooms();
                        } else if parts[1].to_lowercase() == "connections" {
                            list_connections(start_port, end_port);
                        }
                    } else {
                        println!("! Usage: /list <rooms|connections [start_port] [end_port]>")
                    }
                }
                "/join" => {
                    // Fill join room parameters based on inputed fields
                    let mut pswd = String::new();
                    let mut name = String::new();
                    let mut port: u16 = 8080;

                    if parts.len() > 3 {
                        pswd = parts[3].to_string();
                    }
                    if parts.len() > 2 {
                        name = parts[2].to_string();
                    }
                    if parts.len() > 1 {
                        port = parts[1].parse().unwrap();
                    } else {
                        println!("Specity a port to join.");
                    }

                    // join room and send necessary information
                    stream = join_room(port, &name, &pswd, &mut stream, user);
                }
                "/host" => {
                    // Fill host room parameters
                    let mut room_name = String::new();
                    let mut room_pswd = String::new();

                    if parts.len() > 3 {
                        room_pswd = parts[3].to_string();
                    }
                    if parts.len() > 2 {
                        room_name = parts[2].to_string();
                    }
                    if parts.len() > 1 {
                        let room_port = parts[1].parse::<u16>().unwrap();
                        stream = host_room(room_port, &room_name, &room_pswd, &mut stream, user);
                    } else {
                        println!("Specity a port to join.");
                    }

                }
                "/quit" => {
                    match stream {
                        Some(ref mut s) => {
                            println!("Leaving the room...");
                            let _ = s.shutdown(Shutdown::Both);
                        }
                        None => {
                            break;
                        }
                    }
                    stream = None;
                }
                _ => {
                    // send msg if in a room
                    if let Some(ref mut s) = stream {
                        send_msg(s, input.as_str());
                    } else {
                        println!("Not connected to any server. Use /join first.");
                    }

                }
            }
        }
    }
}

fn show_help() {
    println!("Help Menu:");
    println!("* /h, /help\n  Show this help menu\n");
    println!("* /list\n  Show available chat rooms\n");
    println!("* /join <port> [name] [password]\n  Join specified port\n");
    println!("* /host <port> \n  todo\n");
    println!("* /quit\n  Quit your current room/application.\n")
}

fn join_room(port: u16, name: &str, pswd: &str, stream: &mut Option<TcpStream>, user: &User) -> Option<TcpStream> {
    match stream {
        Some(s) => {
            s.shutdown(Shutdown::Both).unwrap();
            println!("Disconnected from room.");
        },
        None => {}
    }

    println!("Connecting...");

    // attempt connection with connect_timeout
    let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));

    match TcpStream::connect_timeout(&addr, ATTEMPT_CONNECT_TIME) {
        Ok(mut s) => {
            println!("Successfully connected to server at {}", addr);

            // Create clone for message receiving
            let stream_clone = s.try_clone().expect("Failed to clone stream");
            
            // New thread for message receive
            thread::spawn(move || {
                receive_messages(stream_clone);
            });
            send_user_info(&mut s, user);

            Some(s)
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            None
        }
    }
}

fn host_room(port: u16, name: &str, pswd: &str, stream: &mut Option<TcpStream>, user: &User) -> Option<TcpStream> {
    let mut rooms_guard = GLOBAL_ROOMS.lock().unwrap();
    
    let room = Room::new(port, name.to_string(), pswd.to_string(), 4);
    let mut room_clone = room.clone();

    thread::spawn(move || {
        room_clone.start_server(port).unwrap();
    });

    rooms_guard.push(room);
    join_room(port, name, pswd, stream, user)

}

fn receive_messages(mut stream: TcpStream) {
    let mut buffer = [0; MSG_SIZE];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("\n! Disconnected from the server");
                break;
            }
            Ok(size) => {
                let msg = String::from_utf8_lossy(&buffer[..size]);
                println!("{}", msg);
            }
            Err(e) => {
                eprintln!("Error reading from server {}: ", e);
                break;
            }
        }
    }

    print!("> ");
    io::stdout().flush().unwrap();
}

fn send_user_info(stream: &mut TcpStream, user: &User) {
    // serialize information with serde json and send as stirng
    let serialized = serde_json::to_string(user).expect("Failed to serialize user info");

    stream.write_all(serialized.as_bytes()).expect("Failed to send user info");
}

fn send_msg(stream: &mut TcpStream, msg: &str) {
    stream.write_all(msg.as_bytes()).expect("Failed to send message");
}

fn list_rooms() {
    let rooms_guard = GLOBAL_ROOMS.lock().unwrap();
    
    if rooms_guard.is_empty() {
        println!("No rooms are open yet. Create one with /host\n")
    }

    for room in &*rooms_guard {
        let formatted_room = format!("-------------------------------\n{}", room.display());
        println!("{}", formatted_room);
    }
}

fn list_connections(start: u16, end: u16) {
    for n in start..end {
        let curr_socket_addr = SocketAddr::from(([0, 0, 0, 0], n));

        if let Ok(_) = TcpListener::bind(curr_socket_addr) {
            println!("Available: {}", curr_socket_addr);
        }
    }
}