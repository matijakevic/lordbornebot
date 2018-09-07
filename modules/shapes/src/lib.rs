extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rusqlite;
#[macro_use]
extern crate lordbornebot_core;

mod data;

use data::*;
use lordbornebot_core::{Config, Message, Module};
use rusqlite::Connection;
use std::boxed::Box;
use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn _create_module(config: &Config) -> *mut Module {
    Box::into_raw(Box::new(Shapes::new(
        Connection::open(&config.database_path).unwrap(),
    )))
}

struct ShapeData {
    stage: i32,
    token: String,
}

impl ShapeData {
    fn reset(&mut self) {
        self.stage = 0;
    }
}

pub struct Shapes {
    connection: Connection,
    channel_shape: HashMap<String, ShapeData>,
}

impl Shapes {
    pub fn new(connection: Connection) -> Shapes {
        Shapes {
            connection,
            channel_shape: HashMap::new(),
        }
    }
}

impl Module for Shapes {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        match message {
            Message::Private(privmsg) => {
                let tokens: Vec<&str> = privmsg.text.split(' ').collect();

                let mut token = String::new();
                let finished = {
                    let instance = self.channel_shape.entry(privmsg.channel.clone()).or_insert(
                        ShapeData {
                            stage: 0,
                            token: String::new(),
                        },
                    );

                    match instance.stage {
                        0 => {
                            if tokens.len() == 3 && tokens[0] == tokens[1] && tokens[1] == tokens[2]
                            {
                                instance.token = tokens[0].to_string();
                                instance.stage += 1;
                                false
                            } else {
                                false
                            }
                        }
                        1 | 3 => if tokens.len() == 1 && tokens[0] == instance.token {
                            instance.stage += 1;
                            false
                        } else {
                            instance.reset();
                            false
                        },
                        2 => if tokens.len() == 2
                            && tokens[0] == tokens[1]
                            && tokens[0] == instance.token
                        {
                            instance.stage += 1;
                            false
                        } else {
                            instance.reset();
                            false
                        },
                        4 => if tokens.len() == 3
                            && tokens[0] == tokens[1]
                            && tokens[1] == tokens[2]
                            && tokens[0] == instance.token
                            && tokens[1] == instance.token
                        {
                            instance.reset();
                            token = instance.token.clone();
                            true
                        } else {
                            instance.reset();
                            false
                        },
                        _ => {
                            instance.reset();
                            false
                        }
                    }
                };

                if finished {
                    match get_points(&self.connection, &privmsg.tags["user-id"]) {
                        Ok(curr_points) => {
                            let new_points = curr_points + 100;
                            match set_points(&self.connection, &privmsg.tags["user-id"], new_points) {
                                Ok(_) => return Some(privmsg!(&privmsg.channel, "{} completed the {} E shape, won 100 points and now has {} points. PagChomp", &privmsg.tags["display-name"], &token, new_points)),
                                Err(e) => {
                                    warn!("{}", e);
                                    return None;
                                }
                            }
                        }
                        Err(e) => {
                            warn!("{}", e);
                            return None;
                        }
                    }
                }

                None
            }
            _ => None,
        }
    }
}
