use std::io::{self, Write};
use std::net::TcpStream;
use std::ptr::null;
use std::thread;
use std::sync::mpsc;

use crate::libs::User;

pub fn start_client() {
    println!("Client started. Type '/h' to view all commands.");

    let user = initialize_user();
    handle_input(&user);
}

fn initialize_user() -> User {
    // impleemnt userproperly
    User {
        user_name: String::from("bob"),
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
            let parts: Vec<&str> = input.split_whitespace().collect();
            match parts[0] {
                "/h" | "/help" => show_help(),
                "/join" => {
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

                    stream = join_room(&server_ip, port, &name, &pswd);
                    if let Some(ref mut s) = stream {
                        send_user_info(s, user);
                    } else {
                        println!("u should never be here. if ur here ur fricked.");
                    }
                },
                "/host" => host_room(),
                _ => {
                    if let Some(ref mut s) = stream {
                        send_msg(s, input);
                        println!("{}", input);
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
}

fn join_room(server_ip: &str, port: u16, name: &str, password: &str) -> Option<TcpStream> {
    let addr = format!("{}:{}", server_ip, port);
    match TcpStream::connect(&addr) {
        Ok(stream) => {
            println!("Successfully connected to server at {}", addr);
            Some(stream)
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            None
        }
    }
}

fn send_user_info(stream: &mut TcpStream, user: &User) {
    let serialized = serde_json::to_string(user).expect("Failed to serialize user info");

    stream.write_all(serialized.as_bytes()).expect("Failed to send user info");
}

fn send_msg(stream: &mut TcpStream, msg: &str) {
    stream.write_all(msg.as_bytes()).expect("Failed to send message");
}

fn host_room() {
    println!("Implementing");
}