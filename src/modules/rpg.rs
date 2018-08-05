use modules::Module;
use rusqlite::{Connection, Error};
use std::str::FromStr;
use twitch::parser::Message;

pub struct User {
    id: String,
    hp: i32,
    strength: i32,
    dexterity: i32,
    weapon_id: Option<i32>,
}

pub struct RPG {
    connection: Connection,
}

impl RPG {
    pub fn new(db_path: &str) -> RPG {
        RPG {
            connection: Connection::open(db_path).unwrap(),
        }
    }

    pub fn create_user(&self, user: &User) -> Result<(), Error> {
        self.connection.execute(
            "INSERT INTO RPGUsers (ID, HP, Strength, Dexterity) VALUES (?,?,?,?)",
            &[&user.id, &user.hp, &user.strength, &user.dexterity],
        )?;
        Ok(())
    }

    pub fn parse_point_allocations(
        params: &Vec<String>,
    ) -> Result<(i32, i32, i32), <i32 as FromStr>::Err> {
        let hp = params[0].parse::<i32>()?;
        let strength = params[1].parse::<i32>()?;
        let dexterity = params[2].parse::<i32>()?;
        Ok((hp, strength, dexterity))
    }
}

impl Module for RPG {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        if let Message::Command(command) = message {
            match command.name.as_ref() {
                "create" => {
                    if let Some(params) = &command.args {
                        if params.len() == 3 {
                            match RPG::parse_point_allocations(&params) {
                                Ok((hp, strength, dexterity)) => {
                                    let sum = hp + strength + dexterity;

                                    if sum != 10 {
                                        return Some(privmsg!(&command.channel, "{}, you have to allocate 10 points, but you've allocated {}.", command.tags["display-name"], sum));
                                    } else {
                                        match self.create_user(&User {
                                            id: command.tags["user-id"].clone(),
                                            hp,
                                            strength,
                                            dexterity,
                                            weapon_id: None,
                                        }) {
                                            Ok(()) => {
                                                return Some(privmsg!(
                                                    &command.channel,
                                                    "Created a new player for {}!",
                                                    command.tags["display-name"]
                                                ));
                                            },
                                            Err(e) => {
                                                
                                            }
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}
