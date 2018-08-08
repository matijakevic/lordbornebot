use data::rpg::*;
use modules::Module;
use rusqlite::Connection;

use std::path::Path;
use std::str::FromStr;
use twitch::parser::{CommandData, Message, PrivateMessage, Response};

const MAX_ALLOCATED_POINTS: i32 = 10;

pub struct RPG {
    connection: Connection,
}

enum PlayerGetBy {
    Username(String),
    ID(String),
}

impl RPG {
    pub fn new(db_path: &Path) -> RPG {
        RPG {
            connection: Connection::open(db_path).unwrap(),
        }
    }

    pub fn parse_point_allocations(args: &Vec<String>) -> Result<Stats, <i32 as FromStr>::Err> {
        let vit = args[0].parse::<i32>()?;
        let str = args[1].parse::<i32>()?;
        let dex = args[2].parse::<i32>()?;
        Ok(Stats { vit, str, dex })
    }

    fn player_info_command(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Response> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if args.len() >= 1 {
            // User specified who's info he/she wants.
            let target = command.args[0].to_lowercase();
            match get_twitch_id(&self.connection, &target) {
                Ok(id) => match load_player(&self.connection, &id) {
                    Ok(Some(player)) => {
                        return Some(Response::Message(privmsg!(
                            &privmsg.channel,
                            "{}, {}'s stats: VIT {}, STR {}, DEX {}, state: HP {}",
                            username,
                            target,
                            player.stats.vit,
                            player.stats.str,
                            player.stats.dex,
                            player.state.hp
                        )))
                    }
                    Ok(None) => {
                        return Some(Response::Message(whisper!(
                            username,
                            "That user doesn't have a character created"
                        )))
                    }
                    Err(e) => {
                        error!("{}", e);
                        return None;
                    }
                },
                Err(_) => {
                    return Some(Response::Message(whisper!(username, "No such user found.")));
                }
            }
        } else {
            // User wants own info.
            match load_player(&self.connection, &privmsg.tags["user-id"]) {
                Ok(Some(player)) => {
                    return Some(Response::Message(privmsg!(
                        &privmsg.channel,
                        "{}'s stats: VIT {}, STR {}, DEX {}, state: HP {}",
                        &username.to_lowercase(),
                        player.stats.vit,
                        player.stats.str,
                        player.stats.dex,
                        player.state.hp
                    )))
                }
                Ok(None) => {
                    return Some(Response::Message(whisper!(
                        username,
                        "That user doesn't have a character created."
                    )))
                }
                Err(e) => {
                    error!("{}", e);
                    return None;
                }
            }
        }
    }

    fn create_command(
        &mut self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Response> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if let Ok(Some(_)) = load_player(&self.connection, &privmsg.tags["user-id"]) {
            return Some(Response::Message(privmsg!(
                &privmsg.channel,
                "{}, your character is already created.",
                username,
            )));
        }

        if args.len() >= 3 {
            match RPG::parse_point_allocations(&args) {
                Ok(stats) => {
                    let sum = stats.vit + stats.str + stats.dex;

                    if sum != MAX_ALLOCATED_POINTS {
                        return Some(Response::Message(whisper!(
                            username,
                            "You have to allocate {} points, but you've allocated {}.",
                            MAX_ALLOCATED_POINTS,
                            sum
                        )));
                    } else {
                        save_player(
                            &self.connection,
                            &privmsg.tags["user-id"],
                            &Player::new(stats),
                        );

                        return Some(Response::Message(privmsg!(
                            &privmsg.channel,
                            "{}, successfully created your character! PagChomp",
                            username
                        )));
                    }
                }
                Err(_) => {
                    return Some(Response::Message(whisper!(
                        username,
                        "Point allocations are numerical values."
                    )));
                }
            }
        } else {
            return Some(Response::Message(whisper!(
                username,
                "Command usage: >>create <vitality> <strength> <dexterity>"
            )));
        }
    }

    fn delete_command(
        &mut self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Response> {
        let username = &privmsg.tags["display-name"];

        match delete_player(&self.connection, &privmsg.tags["user-id"]) {
            Err(e) => {
                error!("{}", e);
                return Some(Response::Message(privmsg!(
                    &privmsg.channel,
                    "{}, failed to delete your character.",
                    username,
                )));
            }
            Ok(_) => {
                return Some(Response::Message(privmsg!(
                    &privmsg.channel,
                    "{}, your character is deleted.",
                    username
                )))
            }
        }
    }

    fn inventory_command_list(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Response> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if args.len() == 2 {
            // User specified what type of items to list
        } else {
            // Assuming to list all items
            match get_twitch_id(&self.connection, &username) {
                Ok(id) => {
                    if let Ok(Some(player)) = load_player(&self.connection, &id) {
                        let inv = &player.inventory;
                        let mut messages = Vec::new();

                        {
                            let mut out = String::new();
                            for inv_item in &inv.bag {
                                out += match inv_item {
                                    Some(item) => &item.name,
                                    None => "<empty>",
                                };
                                out += " ";
                            }
                            messages.push(whisper!(username, "Bag: {}", &out));
                        }

                        {
                            let out = match &inv.weapon {
                                Some(item) => &item.name,
                                None => "<empty>",
                            };
                            messages.push(whisper!(username, "Weapon: {}", &out));
                        }

                        {
                            let mut out = "Ring: ".to_string();
                            out += match &inv.ring {
                                Some(item) => &item.name,
                                None => "<empty>",
                            };
                            out += " Helmet: ";
                            out += match &inv.helmet {
                                Some(item) => &item.name,
                                None => "<empty>",
                            };
                            out += " Chestplate: ";
                            out += match &inv.chestplate {
                                Some(item) => &item.name,
                                None => "<empty>",
                            };
                            out += " Necklace: ";
                            out += match &inv.necklace {
                                Some(item) => &item.name,
                                None => "<empty>",
                            };
                            messages.push(whisper!(username, "{}", &out));
                        }

                        return Some(Response::Messages(messages));
                    }
                    return None;
                }
                Err(_) => {}
            }
        }

        None
    }

    fn inventory_command_info(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Response> {
        None
    }

    fn inventory_command(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Response> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if args.len() < 1 {
            return Some(Response::Message(whisper!(
                username,
                "Command usage: >>inventory <list< weapons|armor|consumables>>|<info <item_name>>"
            )));
        }

        match args[0].as_ref() {
            "list" => return self.inventory_command_list(privmsg, command),
            "info" => return self.inventory_command_info(privmsg, command),
            _ => {
                return Some(Response::Message(whisper!(
                username,
                "Command usage: >>inventory <list<all|weapons|armor|consumables>|info <item_name>>"
            )));
            }
        }
    }
}

impl Module for RPG {
    fn handle_message(&mut self, message: &Message) -> Option<Response> {
        if let Message::Command(privmsg, command) = message {
            return match command.name.as_ref() {
                "delete" => self.delete_command(privmsg, command),
                "create" => self.create_command(privmsg, command),
                "info" => self.player_info_command(privmsg, command),
                "inventory" => self.inventory_command(privmsg, command),
                _ => None,
            };
        }
        None
    }
}
