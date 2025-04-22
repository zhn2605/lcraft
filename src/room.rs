use std::io::{self, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream, TcpListener};
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

use crate::libs::{User, Client};

const MSG_SIZE: usize = 512;

pub fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    listener.set_nonblocking(true)?;

    let clients = Arc::new(Mutex::new(Vec::<Client>::new()));
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");

                let clients_clone = Arc::clone(&clients);

                thread::spawn(move || {
                    handle_client(stream, clients_clone);
                });
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }
            Err(e) => {
                eprintln!("Error accepting client: {}", e);
            }
        }
    }
    Ok(())
}


fn handle_client(mut stream: TcpStream, clients: Arc<Mutex<Vec<Client>>>) {
    let mut buffer = [0; MSG_SIZE];

    match stream.read(&mut buffer) {
        Ok(size) => {
            if size > 0 {
                let user_str = String::from_utf8_lossy(&buffer[..size]);
                let user: User = serde_json::from_str(&user_str).unwrap_or_default();
                
                {
                    let mut clients_lock = clients.lock().unwrap();
                    clients_lock.push(Client {
                        user: user.clone(),
                        stream: stream.try_clone().unwrap(),
                    });
                }
                
                handle_client_messages(stream, &user, clients);

            }
        }
        Err(e) => {
            eprintln!("Error reading from stream {}", e);
        }
    }
}

fn handle_client_messages(mut stream: TcpStream, user: &User, clients: Arc<Mutex<Vec<Client>>>) {
    let mut buffer = [0; MSG_SIZE];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected: {}", user.user_name);
                break;
            }
            Ok(size) => {
                let msg = String::from_utf8_lossy(&buffer[..size]);
                println!("{}: {}", user.user_name, msg);
                
                // process & broadcast msg to other clients
                broadcast_message(&msg, user, &clients);
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }

    let mut clients_lock = clients.lock().unwrap();
    if let Some(pos) = clients_lock.iter().position(|c| c.user.user_name == user.user_name) {
        clients_lock.remove(pos);
    }
}

fn broadcast_message(msg: &str, user: &User, clients: &Arc<Mutex<Vec<Client>>>) {
    let clients_guard = clients.lock().unwrap();

    for client in clients_guard.iter() {
        if client.user == *user {
            continue;
        }

        if let Ok(mut stream_clone) = client.stream.try_clone() {
            let formatted_msg = format!("{}: {}", user.user_name, msg);
            stream_clone
                .write_all(formatted_msg.as_bytes())
                .expect(format!("Failed to send message to {}", client.user.user_name).as_str());
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
