use serde::{Deserialize, Serialize};
use std::io::{self, ErrorKind, Read, Write};
use std::net::{TcpStream, TcpListener};
use std::thread;
use std::sync::{Arc, Mutex};

const MSG_SIZE: usize = 512;


#[derive(Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub user_name: String,
    // pub curr_room: String,
}

pub struct Client {
    pub user: User,
    pub stream: TcpStream,
}

#[derive(Clone)]
pub struct Room {
    room_name: String,
    room_password: String,
    room_port: u16,
    client_count: usize,
    clients: Arc<Mutex<Vec<Client>>>,
    max_client_count: usize,
}

impl Room {
    pub fn new(port: u16, name: String, pswd: String, max: usize) -> Self {
        Self {
            room_name: name,
            room_password: pswd,
            room_port: port,
            client_count: 0,
            clients: Arc::new(Mutex::new(Vec::<Client>::new())),
            max_client_count: max,
        }
    }

    fn set_password(&mut self, pswd: &str) {
        if !pswd.is_empty() && pswd != self.room_password {
            self.room_password = pswd.to_string();
        }
    }

    pub fn display(&self) -> String {
        let mut has_password = false;
        
        if self.room_password != "" {
            has_password = true;
        }
        format!("Room name: {}\nPort: {}\nRoom Capacity: {} / {}\nHas Password: {}\n",
            self.room_name,
            self.room_port,
            self.client_count, self.max_client_count,
            has_password)
    }

    pub fn start_server(&mut self, port: u16) -> io::Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port).as_str())?;
        listener.set_nonblocking(true)?;
        //  check for listener

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New client connected!");
    
                    let clients_clone = Arc::clone(&self.clients);
    
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