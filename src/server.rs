use std::io::{self, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream, TcpListener};
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

use crate::libs::{User, Room};

const MSG_SIZE: usize = 512;

pub fn start_server() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    listener.set_nonblocking(true)?;

    let users = Arc::new(Mutex::new(Vec::<User>::new()));
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected!");

                let users_clone = Arc::clone(&users);

                thread::spawn(move || {
                    handle_client(stream, users_clone);
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


fn handle_client(mut stream: TcpStream, users: Arc<Mutex<Vec<User>>>) {
    let mut buffer = [0; MSG_SIZE];
    let (tx, rx) = mpsc::channel::<String>();

    match stream.read(&mut buffer) {
        Ok(size) => {
            if size > 0 {
                let user_str = String::from_utf8_lossy(&buffer[..size]);
                let user: User = serde_json::from_str(&user_str).unwrap_or_default();
                
                {
                    let mut users_lock = users.lock().unwrap();
                    users_lock.push(user.clone());
                }
                
                handle_client_messages(stream, user, users);

            }
        }
        Err(e) => {
            eprintln!("Error reading from stream {}", e);
        }
    }
}

fn handle_client_messages(mut stream: TcpStream, user: User, users: Arc<Mutex<Vec<User>>>) {
    let mut buffer = [0; MSG_SIZE];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected: {}", user.user_name);
                break;
            }
            Ok(size) => {
                let msg = String::from_utf8_lossy(&buffer[..size]);
                println!("{}", msg);
                
                // process & broadcast msg to other clients
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }

    let mut users_lock = users.lock().unwrap();
    if let Some(pos) = users_lock.iter().position(|u| u.user_name == user.user_name) {
        users_lock.remove(pos);
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
