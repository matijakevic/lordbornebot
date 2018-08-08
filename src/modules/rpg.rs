use bincode::{deserialize_from, serialize_into, Error};
use data::rpg::*;
use modules::Module;
use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;
use twitch::parser::{CommandData, Message, PrivateMessage};

const MAX_ALLOCATED_POINTS: i32 = 10;

#[derive(Serialize, Deserialize)]
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

    pub fn save_data(&self) -> Result<(), Error> {
        let mut file = File::create("rpg_module.dat").unwrap();
        serialize_into(file, self)
    }

    pub fn load_data(&mut self) -> Result<(), Error> {
        let mut file = File::open("rpg_module.dat").unwrap();
        let data: RPG = deserialize_from(file)?;
        self.game = data.game;
        self.mapper = data.mapper;
        Ok(())
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
    ) -> Option<Message> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        let target = if args.len() >= 1 {
            // User specified who's info he/she wants.
            command.args[0].to_lowercase()
        } else {
            // User wants own info.
            username.to_lowercase()
        };

        match self.mapper.get(&target) {
            Some(id) => {
                let player = &self.game.players[id];

                return Some(whisper!(
                    username,
                    "{}'s stats: VIT {}, STR {}, DEX {}, state: HP {}",
                    target,
                    player.stats.vit,
                    player.stats.str,
                    player.stats.dex,
                    player.state.hp
                ));
            }
            None => {
                return Some(whisper!(username, "No such user found."));
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

        if args.len() >= 3 {
            match RPG::parse_point_allocations(&args) {
                Ok(stats) => {
                    let sum = stats.vit + stats.str + stats.dex;

                    if sum != MAX_ALLOCATED_POINTS {
                        return Some(whisper!(
                            username,
                            "You have to allocate {} points, but you've allocated {}.",
                            MAX_ALLOCATED_POINTS,
                            sum
                        ));
                    } else {
                        self.mapper.insert(
                            username.to_lowercase().to_string(),
                            privmsg.tags["user-id"].clone(),
                        );

                        self.game
                            .players
                            .insert(privmsg.tags["user-id"].clone(), Player::new(stats));

                        return Some(privmsg!(
                            &privmsg.channel,
                            "{}, successfully created your character! PagChomp",
                            username
                        ));
                    }
                }
                Err(_) => {
                    return Some(whisper!(
                        username,
                        "Point allocations are numerical values."
                    ));
                }
            }
        } else {
            return Some(whisper!(
                username,
                "Command usage: >>create <vitality> <strength> <dexterity>"
            ));
        }
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
            match self.mapper.get(username) {
                Some(id) => {
                    let player = &self.game.players[id];
                    let inv = &player.inventory;

                    return Some(whisper!(username, "Your inventoryabc"));
                }
                None => {}
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

    fn data_command(&mut self, privmsg: &PrivateMessage, command: &CommandData) -> Option<Message> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if args.len() < 1 {
            return Some(whisper!(username, "Command usage: >>data <save|load>"));
        }

        match args[0].as_ref() {
            "load" => match self.load_data() {
                Ok(()) => {
                    return Some(privmsg!(&privmsg.channel, "Game data loaded!"));
                }
                Err(e) => {
                    error!("{}", e);
                    return None;
                }
            },
            "save" => match self.save_data() {
                Ok(()) => {
                    return Some(privmsg!(&privmsg.channel, "Game data saved!"));
                }
                Err(e) => {
                    error!("{}", e);
                    return None;
                }
            },
            _ => {
                return Some(whisper!(username, "Command usage: >>data <save|load>"));
            }
        }
    }

    fn inventory_command(
        &self,
        privmsg: &PrivateMessage,
        command: &CommandData,
    ) -> Option<Message> {
        let args = &command.args;
        let username = &privmsg.tags["display-name"];

        if args.len() < 1 {
            return Some(whisper!(
                username,
                "Command usage: >>inventory <list<all|weapons|armor|consumables>|info <item_name>>"
            ));
        }

        match args[0].as_ref() {
            "list" => return self.inventory_command_list(privmsg, command),
            "info" => return self.inventory_command_info(privmsg, command),
            _ => {
                return Some(whisper!(
                username,
                "Command usage: >>inventory <list<all|weapons|armor|consumables>|info <item_name>>"
            ));
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
                "inventory" => self.inventory_command(privmsg, command),
                "data" => self.data_command(privmsg, command),
                _ => None,
            };
        }
        None
    }
}
