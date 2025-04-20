use std::io::{self, Write};
use std::net::TcpStream;

pub fn start_client() {
    println!("Client started. Type '/h' to view all commands.");

    handle_input();
}

fn handle_input() {
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

                    join_room(&server_ip, port, &name, &pswd);
                },
                "/host" => host_room(),
                _ => { println!("{}", input); }
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

fn join_room(server_ip: &str, port: u16, name: &str, password: &str) {
    let addr = format!("{}:{}", server_ip, port);
    match TcpStream::connect(&addr) {
        Ok(mut stream) => {
            println!("Successfully connected to server at {}", addr);
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
}

fn host_room() {
    println!("Implementing");
}