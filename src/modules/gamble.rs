use database::points::{get_points, set_points};
use modules::Module;
use rand::random;
use rusqlite::Connection;
use std::path::Path;
use twitch::parser::Command;
use twitch::parser::Message;

pub struct Gamble {
    connection: Connection,
}

impl Gamble {
    pub fn new(db_path: &Path) -> Gamble {
        Gamble {
            connection: Connection::open(db_path).unwrap(),
        }
    }

    fn gamble_command(&self, command: &Command) -> Option<Message> {
        let args = &command.args;

        if args.len() < 1 {
            return None;
        }

        let curr_points = match get_points(&self.connection, &command.tags["user-id"]) {
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
            return Some(privmsg!(
                &command.channel,
                "{}, enter a positive amount of points.",
                &command.tags["display-name"]
            ));
        }

        if amount <= curr_points {
            if random::<f32>() > 0.5 {
                let new_points = curr_points + amount;
                set_points(&self.connection, &command.tags["user-id"], new_points);
                return Some(privmsg!(
                    &command.channel,
                    "{} has won and now has {} points.",
                    &command.tags["display-name"],
                    new_points
                ));
            } else {
                let new_points = curr_points - amount;
                set_points(&self.connection, &command.tags["user-id"], new_points);
                return Some(privmsg!(
                    &command.channel,
                    "{} has lost and now has {} points.",
                    &command.tags["display-name"],
                    new_points
                ));
            }
        } else {
            return Some(privmsg!(
                &command.channel,
                "{}, you don't have enough points for this roulette.",
                &command.tags["display-name"]
            ));
        }
    }
}

impl Module for Gamble {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        match message {
            Message::Command(command) => match command.name.as_ref() {
                "gamble" | "roulette" => self.gamble_command(&command),
                _ => return None,
            },
            _ => return None,
        }
    }
}
