use data::points::{get_points, get_points_by_username};
use modules::Module;
use rusqlite::Connection;
use std::path::Path;
use twitch::parser::{CommandData, Message, PrivateMessage, Response};

pub struct Points {
    connection: Connection,
}

impl Points {
    pub fn new(db_path: &Path) -> Points {
        Points {
            connection: Connection::open(db_path).unwrap(),
        }
    }

    fn points_command(&self, privmsg: &PrivateMessage, command: &CommandData) -> Option<Response> {
        let args = &command.args;

        if args.len() < 1 {
            let points = match get_points(&self.connection, &privmsg.tags["user-id"]) {
                Ok(points) => points,
                Err(e) => {
                    warn!("{}", e);
                    return None;
                }
            };

            return Some(Response::Message(privmsg!(
                &privmsg.channel,
                "{}, you have {} points.",
                &privmsg.tags["display-name"],
                points
            )));
        } else {
            match get_points_by_username(&self.connection, &args[0]) {
                Ok(points) => {
                    return Some(Response::Message(privmsg!(
                        &privmsg.channel,
                        "{}, {} has {} points.",
                        &privmsg.tags["display-name"],
                        args[0],
                        points
                    )))
                }
                Err(_) => {
                    return Some(Response::Message(privmsg!(
                        &privmsg.channel,
                        "{}, that user doesn't exist yet in the database.",
                        &privmsg.tags["display-name"]
                    )))
                }
            }
        }
    }
}

impl Module for Points {
    fn handle_message(&mut self, message: &Message) -> Option<Response> {
        match message {
            Message::Command(privmsg, command) => match command.name.as_ref() {
                "points" => self.points_command(&privmsg, &command),
                _ => return None,
            },
            _ => return None,
        }
    }
}
