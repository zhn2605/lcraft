use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream, Shutdown};
use std::thread;
use std::time::Duration;

use crate::libs::User;

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
    // Initialize user fields

    print!("Enter user name:\n> ");
        io::stdout().flush().unwrap();
    
    // User name
    let mut input = String::new();
    
    // Username
    loop {
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if !input.is_empty() && input.chars().all(|c| c.is_alphanumeric()) {
            break;
        } else {
            println!("Must be alpha numeric.\n Enter user name:\n> ");
        }
    }

    // other fields

    User {
        user_name: String::from(input),
    }
}

fn handle_input(user: &User) {
    let mut stream: Option<TcpStream> = None;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if !input.is_empty() {
            // parse commands
            let parts: Vec<&str> = input.split_whitespace().collect();
            match parts[0] {
                "/h" | "/help" => show_help(),
                "/list" => {println!("Implementing");},
                "/join" => {
                    // Fill join room parameters based on inputed fields
                    let mut server_ip = String::new();
                    let mut pswd = String::new();
                    let mut name = String::new();
                    let mut port: u16 = 8080;

                    if parts.len() > 4 {
                        pswd = parts[4].to_string();
                    }
                    if parts.len() > 3 {
                        name = parts[3].to_string();
                    }
                    if parts.len() > 2 {
                        port = parts[2].parse().unwrap();
                        server_ip = parts[1].to_string();
                    } else {
                        println!("Specity a port to join.");
                    }

                    // join room and send necessary information
                    stream = join_room(&server_ip, port, &name, &pswd);
                    if let Some(ref mut s) = stream {
                        send_user_info(s, user);
                    } else {
                        println!("No stream found.");
                    }
                },
                "/host" => host_room(),
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
                        send_msg(s, input);
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
    println!("* /join <server_ip> <port> [name] [password]\n  Join specified port\n");
    println!("* /host\n  todo\n");
    println!("* /quit\n  Quit your current room/application.\n")
}

fn join_room(server_ip: &str, port: u16, name: &str, password: &str) -> Option<TcpStream> {
    println!("Connecting...");

    // attempt connection with connect_timeout
    let ip: Ipv4Addr = server_ip.parse().expect("Invalid IP address");
    let addr: SocketAddr = SocketAddr::V4(SocketAddrV4::new(ip, port));

    match TcpStream::connect_timeout(&addr, ATTEMPT_CONNECT_TIME) {
        Ok(stream) => {
            println!("Successfully connected to server at {}", addr);

            // Create clone for message receiving
            let stream_clone = stream.try_clone().expect("Failed to clone stream");
            
            // New thread for message receive
            thread::spawn(move || {
                receive_messages(stream_clone);
            });

            Some(stream)
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            None
        }
    }
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

fn host_room() {
    println!("Implementing");
}