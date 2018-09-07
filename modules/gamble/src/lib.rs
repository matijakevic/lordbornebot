extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rand;
extern crate rusqlite;
#[macro_use]
extern crate lordbornebot_core;

mod data;

use data::*;
use lordbornebot_core::{CommandData, Config, Message, Module, PrivateMessage};
use rand::random;
use rusqlite::Connection;
use std::boxed::Box;

#[no_mangle]
pub extern "C" fn _create_module(config: &Config) -> *mut Module {
    Box::into_raw(Box::new(Gamble::new(
        Connection::open(&config.database_path).unwrap(),
    )))
}

pub struct Gamble {
    connection: Connection,
}

impl Gamble {
    pub fn new(connection: Connection) -> Gamble {
        Gamble { connection }
    }

    fn gamble_command(&self, privmsg: &PrivateMessage, command: &CommandData) -> Option<Message> {
        let args = &command.args;

        if args.is_empty() {
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
            // Have to do it like this until custom message interpolation/templating system.
            let (new_points, message) = if random::<f32>() > 0.5 {
                let new_points = curr_points - amount;
                (
                    new_points,
                    privmsg!(
                        &privmsg.channel,
                        "{} has lost and now has {} points. FeelsWeirdMan",
                        &privmsg.tags["display-name"],
                        new_points
                    ),
                )
            } else {
                let new_points = curr_points + amount;
                (
                    new_points,
                    privmsg!(
                        &privmsg.channel,
                        "{} has won and now has {} points. PagChomp",
                        &privmsg.tags["display-name"],
                        new_points
                    ),
                )
            };

            match set_points(&self.connection, &privmsg.tags["user-id"], new_points) {
                Ok(_) => Some(message),
                Err(e) => {
                    warn!("{}", e);
                    None
                }
            }
        } else {
            Some(whisper!(
                &privmsg.tags["display-name"],
                "{}, you don't have enough points for this roulette.",
                &privmsg.tags["display-name"]
            ))
        }
    }
}

impl Module for Gamble {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        match message {
            Message::Command(privmsg, command) => match command.name.as_ref() {
                "gamble" | "roulette" => self.gamble_command(&privmsg, &command),
                _ => None,
            },
            _ => None,
        }
    }
}
