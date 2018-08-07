use data::rpg::{create_player, get_all_player_inventory, get_player_info};
use modules::Module;
use rusqlite::{Connection, Error};
use std::path::Path;
use std::str::FromStr;
use twitch::parser::{CommandData, Message, PrivateMessage};

const MAX_ALLOCATED_POINTS: i32 = 10;

pub struct RPG {
    connection: Connection,
}

impl RPG {
    pub fn new(db_path: &Path) -> RPG {
        RPG {
            connection: Connection::open(db_path).unwrap(),
        }
    }

    pub fn parse_point_allocations(
        params: &Vec<String>,
    ) -> Result<(i32, i32, i32), <i32 as FromStr>::Err> {
        let hp = params[0].parse::<i32>()?;
        let strength = params[1].parse::<i32>()?;
        let dexterity = params[2].parse::<i32>()?;
        Ok((hp, strength, dexterity))
    }

    fn player_info_command(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let params = &command.args;

        let username = if params.len() >= 1 {
            &command.args[0]
        } else {
            &privmsg.tags["display-name"]
        };

        match get_player_info(&self.connection, username) {
            Ok((stats, state)) => {
                return Some(privmsg!(
                    &privmsg.channel,
                    "{}'s stats: VIT {}, STR {}, DEX {}, state: HP {}",
                    username,
                    stats.vitality,
                    stats.strength,
                    stats.dexterity,
                    state.hp
                ));
            }
            Err(e) => {
                warn!("{}", e);
                return None;
            }
        }
    }

    fn create_command(&self, privmsg: &PrivateMessage, command: &CommandData) -> Option<Message> {
        let params = &command.args;
        if params.len() >= 3 {
            match RPG::parse_point_allocations(&params) {
                Ok((vitality, strength, dexterity)) => {
                    let sum = vitality + strength + dexterity;

                    if sum != MAX_ALLOCATED_POINTS {
                        return Some(privmsg!(
                            &privmsg.channel,
                            "{}, you have to allocate {} points, but you've allocated {}.",
                            privmsg.tags["display-name"],
                            MAX_ALLOCATED_POINTS,
                            sum
                        ));
                    } else {
                        match create_player(
                            &self.connection,
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
                            Err(e) => {
                                warn!("{}", e);
                                return None;
                            }
                        }
                    }
                }
                Err(_) => {
                    return Some(privmsg!(
                        &privmsg.channel,
                        "{}, command usage: >>create <vitality> <strength> <dexterity>",
                        privmsg.tags["display-name"]
                    ));
                }
            }
        } else {
            return Some(privmsg!(
                &privmsg.channel,
                "{}, command usage: >>create <vitality> <strength> <dexterity>",
                privmsg.tags["display-name"]
            ));
        }
    }

    fn inventory_command_list(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let args = &command.args;

        if args.len() == 2 {
            // User specified what type of items to list
        } else {
            // Assuming to list all items
            match get_all_player_inventory(&self.connection, &privmsg.tags["user-id"]) {
                Ok(items) => {
                    return Some(privmsg!(
                        &privmsg.channel,
                        "{}, command usage: >>create <vitality> <strength> <dexterity>",
                        privmsg.tags["display-name"]
                    ))
                }
                Err(e) => {
                    warn!("{}", e);
                    return None;
                }
            }
        }

        None
    }

    fn inventory_command_info(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        None
    }

    fn inventory_command(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let args = &command.args;

        if args.len() < 1 {
            return Some(privmsg!(
                &privmsg.channel,
                "{}, command usage: >>inventory <list<all|weapons|armor|consumables>|info <item_name>>",
                privmsg.tags["display-name"]
            ));
        }

        match args[0].as_ref() {
            "list" => return self.inventory_command_list(privmsg, command),
            "info" => return self.inventory_command_info(privmsg, command),
            _ => {
                return Some(privmsg!(
                    &privmsg.channel,
                    "{}, command usage: >>create <vitality> <strength> <dexterity>",
                    privmsg.tags["display-name"]
                ))
            }
        }
    }
}

impl Module for RPG {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        if let Message::Command(privmsg, command) = message {
            return match command.name.as_ref() {
                "create" => self.create_command(privmsg, command),
                "info" => self.player_info_command(privmsg, command),
                // "inventory" => self.inventory_command(privmsg, command),
                _ => None,
            };
        }
        None
    }
}
