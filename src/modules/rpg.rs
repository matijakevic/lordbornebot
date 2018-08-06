use modules::Module;
use rusqlite::{Connection, Error};
use std::path::Path;
use std::str::FromStr;
use twitch::parser::Message;

pub struct RPG {
    connection: Connection,
}

impl RPG {
    pub fn new(db_path: &Path) -> RPG {
        RPG {
            connection: Connection::open(db_path).unwrap(),
        }
    }

    pub fn create_player(&self, twitch_id: &str, stats: (i32, i32, i32)) -> Result<(), Error> {
        self.connection
            .execute("INSERT INTO Players (UserID) VALUES (?)", &[&twitch_id])?;
        self.connection.execute(
            "INSERT INTO Stats (PlayerID, Vitality, Strength, Dexterity) VALUES (?,?,?,?)",
            &[&twitch_id, &stats.0, &stats.1, &stats.2],
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
        if let Message::Command(privmsg, command) = message {
            match command.name.as_ref() {
                "create" => {
                    let params = &command.args;
                    if params.len() >= 3 {
                        match RPG::parse_point_allocations(&params) {
                            Ok((vitality, strength, dexterity)) => {
                                let sum = vitality + strength + dexterity;

                                if sum != 10 {
                                    return Some(privmsg!(&privmsg.channel, "{}, you have to allocate 10 points, but you've allocated {}.", privmsg.tags["display-name"], sum));
                                } else {
                                    match self.create_player(
                                        &privmsg.tags["user-id"],
                                        (vitality, strength, dexterity),
                                    ) {
                                        Ok(()) => {
                                            return Some(privmsg!(
                                                &privmsg.channel,
                                                "Created a new player for {}!",
                                                privmsg.tags["display-name"]
                                            ));
                                        }
                                        Err(e) => warn!("{}", e),
                                    }
                                }
                            }
                            Err(_) => {}
                        }
                    } else {
                        return Some(privmsg!(
                            &privmsg.channel,
                            "{}, command usage: >>create <vitality> <strength> <dexterity>",
                            privmsg.tags["display-name"]
                        ));
                    }
                }
                _ => {}
            }
        }
        None
    }
}
