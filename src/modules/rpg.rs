use rusqlite::Connection;
use twitch::parser::Message;
use modules::Module;
use std::path::Path;
use twitch::parser::Command;
use rand::random;

pub struct RPG {
    connection: Connection
}

impl RPG {
    pub fn new(db_path: &str) -> RPG {
        RPG {
            connection: Connection::open(db_path).unwrap()
        }
    }
}

impl Module for RPG {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        None
    }
}