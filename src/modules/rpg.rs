use data::rpg::*;
use modules::Module;
use rusqlite::Connection;

use std::path::Path;
use std::str::FromStr;
use twitch::parser::{CommandData, Message, PrivateMessage, Response};

const MAX_ALLOCATED_POINTS: i32 = 10;

macro_rules! print_inventory_item {
    ($item:expr, $var:expr, $equipped:expr) => {
        if let Some(item) = $item {
            $var += &item.name;
            if $equipped {
                $var += " (equipped)";
            }
            $var += ", ";
        }
    };
}

pub struct RPG {
    connection: Connection,
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
                    if stats.vit <= 0 || stats.dex <= 0 || stats.str <= 0 {
                        return Some(Response::Message(whisper!(
                            username,
                            "Point allocations must be positive numbers."
                        )));
                    }

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
                        let mut out = String::new();

                        for inv_item in &inv.bag {
                            print_inventory_item!(inv_item, out, false);
                        }

                        print_inventory_item!(&inv.weapon, out, true);
                        print_inventory_item!(&inv.ring, out, true);
                        print_inventory_item!(&inv.helmet, out, true);
                        print_inventory_item!(&inv.chestplate, out, true);
                        print_inventory_item!(&inv.necklace, out, true);

                        return Some(Response::Message(whisper!(
                            username,
                            "Your inventory: {}",
                            out.trim_right_matches(", ")
                        )));
                    }
                    return None;
                }
                Err(_) => {}
            }
        }

        None
    }

    fn equip_command() {}

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
