use rusqlite::Connection;
use twitch::parser::Message;
use modules::Module;
use std::path::Path;
use twitch::parser::Command;
use rand::random;

pub struct Gamble {
    connection: Connection
}

impl Gamble {
    pub fn new(db_path: &str) -> Gamble {
        Gamble {
            connection: Connection::open(db_path).unwrap()
        }
    }

    fn get_points(&self, id: &str) -> i32 {
        self.connection.query_row("SELECT Points FROM `Users` WHERE TwitchID=? LIMIT 1", &[&id], |row| {
            row.get(0)
        }).unwrap()
    }

    fn set_points(&self, id: &str, points: i32) {
        self.connection.execute("UPDATE `Users` SET Points=? WHERE TwitchID=?", &[&points, &id]).unwrap();
    }

    fn roulette_command(&self, command: &Command) -> Option<Message> {
        if let Some(args) = &command.args {
            let curr_points = self.get_points(&command.tags["user-id"]);

            let amount = if let Ok(amount) = args[0].parse::<i32>() {
                amount
            } else if args[0] == "all" {
                curr_points
            } else {
                return None;
            };

            if amount > 0 && amount <= curr_points {
                if random::<f32>() > 0.5 {
                    let new_points = curr_points + amount;
                    self.set_points(&command.tags["user-id"], new_points);
                    return Some(privmsg!(&command.channel, "{} won and now has {} points.", &command.tags["display-name"], new_points));
                } else {
                    let new_points = curr_points - amount;
                    self.set_points(&command.tags["user-id"], new_points);
                    return Some(privmsg!(&command.channel, "{} lost and now has {} points.", &command.tags["display-name"], new_points));
                }
            } else {
                return Some(privmsg!(&command.channel, "{}, you don't have enough points for this roulette.", &command.tags["display-name"]));
            }
        }
        return None;
    }
}

impl Module for Gamble {
    fn handle_message(&self, message: &Message) -> Option<Message> {
        match message {
            Message::Command(command) => {
                match command.name.as_ref() {
                    "roulette" => self.roulette_command(&command),
                    _ => return None
                }
            }
            _ => return None
        }
    }
}