use data::points::{get_points, get_points_by_username};
use modules::Module;
use rusqlite::Connection;
use twitch::parser::{CommandData, Message, PrivateMessage};

pub struct Points {
    connection: Connection,
}

impl Points {
    pub fn new(connection: Connection) -> Points {
        Points { connection }
    }

    fn points_command(&self, privmsg: &PrivateMessage, command: &CommandData) -> Option<Message> {
        let args = &command.args;

        if args.is_empty() {
            let points = match get_points(&self.connection, &privmsg.tags["user-id"]) {
                Ok(points) => points,
                Err(e) => {
                    warn!("{}", e);
                    return None;
                }
            };

            Some(privmsg!(
                &privmsg.channel,
                "{}, you have {} points.",
                &privmsg.tags["display-name"],
                points
            ))
        } else {
            match get_points_by_username(&self.connection, &args[0]) {
                Ok(points) => {
                    Some(privmsg!(
                        &privmsg.channel,
                        "{}, {} has {} points.",
                        &privmsg.tags["display-name"],
                        args[0],
                        points
                    ))
                }
                Err(_) => {
                    Some(privmsg!(
                        &privmsg.channel,
                        "{}, that user doesn't exist yet in the database.",
                        &privmsg.tags["display-name"]
                    ))
                }
            }
        }
    }
}

impl Module for Points {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        match message {
            Message::Command(privmsg, command) => match command.name.as_ref() {
                "points" => self.points_command(&privmsg, &command),
                _ => None,
            },
            _ => None,
        }
    }
}
