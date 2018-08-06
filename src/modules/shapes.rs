use modules::Module;
use rusqlite::Connection;
use std::collections::HashMap;
use std::path::Path;
use twitch::parser::Message;

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
    pub fn new(db_path: &Path) -> Shapes {
        Shapes {
            connection: Connection::open(db_path).unwrap(),
            channel_shape: HashMap::new(),
        }
    }

    fn get_points(&self, id: &str) -> i32 {
        self.connection
            .query_row(
                "SELECT Points FROM `Users` WHERE TwitchID=? LIMIT 1",
                &[&id],
                |row| row.get(0),
            )
            .unwrap()
    }

    fn set_points(&self, id: &str, points: i32) {
        self.connection
            .execute(
                "UPDATE `Users` SET Points=? WHERE TwitchID=?",
                &[&points, &id],
            )
            .unwrap();
    }
}

impl Module for Shapes {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        match message {
            Message::Private(privmsg) => {
                let tokens: Vec<&str> = privmsg.text.split(" ").collect();

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
                    let curr_points = self.get_points(&privmsg.tags["user-id"]);
                    let new_points = curr_points + 100;
                    self.set_points(&privmsg.tags["user-id"], new_points);
                    return Some(privmsg!(&privmsg.channel, "{} completed the {} E shape, won 100 points and now has {} points PagChomp", &privmsg.tags["display-name"], &token, new_points));
                }

                return None;
            }
            _ => return None,
        }
    }
}
