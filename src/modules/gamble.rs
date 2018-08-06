use database::points::{get_points, set_points};
use modules::Module;
use rand::random;
use rusqlite::Connection;
use std::path::Path;
use twitch::parser::{CommandData, Message, PrivateMessage};

pub struct Gamble {
    connection: Connection,
}

impl Gamble {
    pub fn new(db_path: &Path) -> Gamble {
        Gamble {
            connection: Connection::open(db_path).unwrap(),
        }
    }

    fn gamble_command(&self, privmsg: &PrivateMessage, command: &CommandData) -> Option<Message> {
        let args = &command.args;

        if args.len() < 1 {
            return None;
        }

        let curr_points = match get_points(&self.connection, &privmsg.tags["user-id"]) {
            Ok(points) => points,
            Err(e) => {
                warn!("{}", e);
                return None;
            }
        };

        let amount = if let Ok(amount) = args[0].parse::<i32>() {
            amount
        } else if args[0] == "all" {
            curr_points
        } else {
            return None;
        };

        if amount <= 0 {
            return Some(whisper!(
                &privmsg.tags["display-name"],
                "{}, please enter a positive amount of points.",
                &privmsg.tags["display-name"]
            ));
        }

        if amount <= curr_points {
            let new_points = if random::<f32>() > 0.5 {
                curr_points + amount
            } else {
                curr_points - amount
            };

            match set_points(&self.connection, &privmsg.tags["user-id"], new_points) {
                Ok(_) => {
                    return Some(privmsg!(
                        &privmsg.channel,
                        "{} has lost and now has {} points.",
                        &privmsg.tags["display-name"],
                        new_points
                    ));
                }
                Err(e) => {
                    warn!("{}", e);
                    return None;
                }
            }
        } else {
            return Some(whisper!(
                &privmsg.tags["display-name"],
                "{}, you don't have enough points for this roulette.",
                &privmsg.tags["display-name"]
            ));
        }
    }
}

impl Module for Gamble {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        match message {
            Message::Command(privmsg, command) => match command.name.as_ref() {
                "gamble" | "roulette" => self.gamble_command(&privmsg, &command),
                _ => return None,
            },
            _ => return None,
        }
    }
}
