use database::points::{get_points, get_points_by_username};
use modules::Module;
use rusqlite::Connection;
use std::path::Path;
use twitch::parser::Command;
use twitch::parser::Message;

pub struct Points {
    connection: Connection,
}

impl Points {
    pub fn new(db_path: &Path) -> Points {
        Points {
            connection: Connection::open(db_path).unwrap(),
        }
    }

    fn points_command(&self, command: &Command) -> Option<Message> {
        let args = &command.args;

        if args.len() < 1 {
            let points = match get_points(&self.connection, &command.tags["user-id"]) {
                Ok(points) => points,
                Err(e) => {
                    warn!("{}", e);
                    return None;
                }
            };

            return Some(privmsg!(
                &command.channel,
                "{}, you have {} points.",
                &command.tags["display-name"],
                points
            ));
        } else {
            match get_points_by_username(&self.connection, &args[0]) {
                Ok(points) => {
                    return Some(privmsg!(
                        &command.channel,
                        "{}, {} has {} points.",
                        &command.tags["display-name"],
                        args[0],
                        points
                    ))
                }
                Err(_) => {
                    return Some(privmsg!(
                        &command.channel,
                        "{}, that user doesn't exist yet in the database.",
                        &command.tags["display-name"]
                    ))
                }
            }
        }
    }
}

impl Module for Points {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        match message {
            Message::Command(command) => match command.name.as_ref() {
                "points" => self.points_command(&command),
                _ => return None,
            },
            _ => return None,
        }
    }
}
