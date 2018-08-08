use data::rpg::*;
use modules::Module;
use std::collections::HashMap;
use std::str::FromStr;
use twitch::parser::{CommandData, Message, PrivateMessage};

const MAX_ALLOCATED_POINTS: i32 = 10;

pub struct RPG {
    game: Game,
    mapper: HashMap<String, String>, // Maps Twitch usernames to user IDs.
}

impl RPG {
    pub fn new() -> RPG {
        RPG {
            game: Game::new(),
            mapper: HashMap::new(),
        }
    }

    pub fn parse_point_allocations(params: &Vec<String>) -> Result<Stats, <i32 as FromStr>::Err> {
        let vit = params[0].parse::<i32>()?;
        let str = params[1].parse::<i32>()?;
        let dex = params[2].parse::<i32>()?;
        Ok(Stats { vit, str, dex })
    }

    fn player_info_command(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let params = &command.args;

        let username = if params.len() >= 1 {
            command.args[0].clone()
        } else {
            privmsg.tags["display-name"].to_lowercase()
        };

        match self.mapper.get(&username) {
            Some(id) => {
                let player = &self.game.players[id];

                return Some(privmsg!(
                    &privmsg.channel,
                    "{}'s stats: VIT {}, STR {}, DEX {}, state: HP {}",
                    username,
                    player.stats.vit,
                    player.stats.str,
                    player.stats.dex,
                    player.state.hp
                ));
            }
            None => {
                return Some(privmsg!(
                    &privmsg.channel,
                    "{}, no such user found.",
                    privmsg.tags["display-name"]
                ));
            }
        }
    }

    fn create_command(
        &mut self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let params = &command.args;
        if params.len() >= 3 {
            match RPG::parse_point_allocations(&params) {
                Ok(stats) => {
                    let sum = stats.vit + stats.str + stats.dex;

                    if sum != MAX_ALLOCATED_POINTS {
                        return Some(privmsg!(
                            &privmsg.channel,
                            "{}, you have to allocate {} points, but you've allocated {}.",
                            privmsg.tags["display-name"],
                            MAX_ALLOCATED_POINTS,
                            sum
                        ));
                    } else {
                        self.mapper.insert(
                            privmsg.tags["display-name"].to_lowercase().to_string(),
                            privmsg.tags["user-id"].clone(),
                        );

                        self.game
                            .players
                            .insert(privmsg.tags["user-id"].clone(), Player::new(stats));

                        return Some(privmsg!(
                            &privmsg.channel,
                            "{}, successfully created your character! PagChomp",
                            privmsg.tags["display-name"]
                        ));
                    }
                }
                Err(_) => {
                    return Some(privmsg!(
                        &privmsg.channel,
                        "{}, point allocations are numerical values.",
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
