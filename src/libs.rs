use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize)]

pub struct User {
    pub user_name: String,
    // pub curr_room: String,
}

#[derive(Clone)]
pub struct Room {
    room_name: String,
    room_password: String,
    room_port: u16,
    user_count: usize,
    users: Vec<User>,
    max_user_count: usize,
}

impl Room {
    fn new(room_name: String, room_port: u16, max_user_count: usize) -> Self {
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

    fn add_user(&mut self, user: User) {
        if self.user_count < self.max_user_count {
            self.users.push(user);
            self.user_count += 1;
            print!("Room capacity: ");
        } else {
            print!("Room is at full capacity: ");
        }
        println!("{}/{}", self.user_count, self.max_user_count);
    }

    fn remove_user(&mut self, name: &str) {
        self.users.retain(|user| user.user_name != name);
        self.user_count -= 1;
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }
}