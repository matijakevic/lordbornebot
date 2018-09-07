use data::rpg::*;
use rusqlite::Connection;

use lordbornebot_core::{CommandData, Message, Module, PrivateMessage};
use std::path::Path;
use std::str::FromStr;

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

    pub fn parse_point_allocations(args: &[String]) -> Result<Stats, <i32 as FromStr>::Err> {
        let vit = args[0].parse::<i32>()?;
        let str = args[1].parse::<i32>()?;
        let dex = args[2].parse::<i32>()?;
        Ok(Stats { vit, str, dex })
    }

    fn player_info_command(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if !args.is_empty() {
            // User specified who's info he/she wants.
            let target = command.args[0].to_lowercase();
            match get_twitch_id(&self.connection, &target) {
                Ok(id) => match load_player(&self.connection, &id) {
                    Ok(Some(player)) => {
                        let dmg = player.get_damage();
                        Some(privmsg!(
                            &privmsg.channel,
                            "{}'s stats: VIT {}, STR {}, DEX {}; HP {}, DMG: {} ({})",
                            target,
                            player.stats.vit,
                            player.stats.str,
                            player.stats.dex,
                            player.state.hp,
                            dmg.0,
                            dmg.1
                        ))
                    }
                    Ok(None) => Some(whisper!(
                        username,
                        "That user doesn't have a character created"
                    )),
                    Err(e) => {
                        error!("{}", e);
                        None
                    }
                },
                Err(_) => Some(whisper!(username, "No such user found.")),
            }
        } else {
            // User wants own info.
            match load_player(&self.connection, &privmsg.tags["user-id"]) {
                Ok(Some(player)) => {
                    let dmg = player.get_damage();
                    Some(privmsg!(
                        &privmsg.channel,
                        "{}'s stats: VIT {}, STR {}, DEX {}; HP {}, DMG: {} ({})",
                        username.to_lowercase(),
                        player.stats.vit,
                        player.stats.str,
                        player.stats.dex,
                        player.state.hp,
                        dmg.0,
                        dmg.1
                    ))
                }
                Ok(None) => Some(whisper!(
                    username,
                    "That user doesn't have a character created."
                )),
                Err(e) => {
                    error!("{}", e);
                    None
                }
            }
        }
    }

    fn create_command(
        &mut self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if let Ok(Some(_)) = load_player(&self.connection, &privmsg.tags["user-id"]) {
            return Some(privmsg!(
                &privmsg.channel,
                "{}, your character is already created.",
                username,
            ));
        }

        if args.len() >= 3 {
            match RPG::parse_point_allocations(&args) {
                Ok(stats) => {
                    if stats.vit <= 0
                        || stats.vit > 8
                        || stats.dex <= 0
                        || stats.dex > 8
                        || stats.str <= 0
                        || stats.str > 8
                    {
                        return Some(whisper!(
                            username,
                            "Point allocations must be values from 1 to 8."
                        ));
                    }

                    let sum = stats.vit + stats.dex + stats.str;

                    if sum != MAX_ALLOCATED_POINTS {
                        Some(whisper!(
                            username,
                            "You have to allocate {} points, but you've allocated {}.",
                            MAX_ALLOCATED_POINTS,
                            sum
                        ))
                    } else {
                        save_player(
                            &self.connection,
                            &privmsg.tags["user-id"],
                            &Player::new(stats),
                        );

                        Some(privmsg!(
                            &privmsg.channel,
                            "{} successfully created a character! PagChomp",
                            username
                        ))
                    }
                }
                Err(_) => Some(whisper!(
                    username,
                    "Point allocations must be values from 1 to 8."
                )),
            }
        } else {
            Some(whisper!(
                username,
                "Command usage: >>create <vitality> <strength> <dexterity>"
            ))
        }
    }

    fn delete_command(
        &mut self,
        privmsg: &PrivateMessage,
        _command: &CommandData,
    ) -> Option<Message> {
        let username = &privmsg.tags["display-name"];

        match delete_player(&self.connection, &privmsg.tags["user-id"]) {
            Err(e) => {
                error!("{}", e);
                Some(privmsg!(
                    &privmsg.channel,
                    "{}, failed to delete your character.",
                    username,
                ))
            }
            Ok(_) => Some(privmsg!(
                &privmsg.channel,
                "{}, your character is deleted.",
                username
            )),
        }
    }

    fn print_inventory_item<T>(item: &Option<InventoryItem<T>>, equipped: bool) -> String {
        if let Some(item) = item {
            let mut out = item.name.to_string();
            if equipped {
                out += " (equipped)";
            }
            out += ", ";
            return out;
        }
        String::new()
    }

    fn inventory_command_list(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
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
                            out += &RPG::print_inventory_item(inv_item, false);
                        }

                        out += &RPG::print_inventory_item(&inv.weapon, true);
                        /*out += &RPG::print_inventory_item(&inv.ring, true);
                        out += &RPG::print_inventory_item(&inv.helmet, true);
                        out += &RPG::print_inventory_item(&inv.necklace, true);
                        out += &RPG::print_inventory_item(&inv.chestplate, true);*/

                        return Some(whisper!(
                            username,
                            "Your inventory: {}",
                            out.trim_right_matches(", ")
                        ));
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
        _privmsg: &PrivateMessage,
        _command: &CommandData,
    ) -> Option<Message> {
        None
    }

    fn inventory_command(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if args.is_empty() {
            return Some(whisper!(
                username,
                "Command usage: >>inventory <list< weapons|armor|consumables>>|<info <item_name>>"
            ));
        }

        match args[0].as_ref() {
            "list" => self.inventory_command_list(privmsg, command),
            "info" => self.inventory_command_info(privmsg, command),
            _ => Some(whisper!(
                username,
                "Command usage: >>inventory <list<all|weapons|armor|consumables>|info <item_name>>"
            )),
        }
    }
}

impl Module for RPG {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
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
