extern crate chrono;
extern crate rand;
extern crate regex;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod twitch;
mod modules;
mod util;

use modules::gamble::Gamble;
use modules::rpg::RPG;
use modules::shapes::Shapes;
use modules::Module;
use rusqlite::Connection;
use std::env::var_os;
use std::ffi::OsString;
use twitch::client::Client;
use twitch::parser::{Message, Parser};
use util::load_json_from_file;

#[derive(Deserialize)]
struct Config {
    oauth: String,
    nickname: String,
    command_prefix: String,
    database_path: String,
    channels: Vec<String>,
}

fn forward_to_modules(modules: &mut Vec<Box<Module>>, message: &Message, client: &mut Client) {
    for module in modules {
        if let Some(out_message) = module.handle_message(&message) {
            match Parser::encode(&out_message) {
                Ok(raw_message) => client.send_line(&raw_message),
                Err(e) => println!("{}", e),
            }
        }
    }
}

fn init_modules(config: &Config, modules: &mut Vec<Box<Module>>) {
    let gamble_module = Gamble::new(&config.database_path);
    let shapes_module = Shapes::new(&config.database_path);
    let rpg_module = RPG::new(&config.database_path);

    modules.push(Box::new(gamble_module));
    modules.push(Box::new(shapes_module));
    modules.push(Box::new(rpg_module));
}

fn load_config() -> Config {
    let path = match var_os("BOT_CONFIG_PATH") {
        Some(path) => OsString::from(path),
        None => OsString::from("config.json"),
    };

    match load_json_from_file(path) {
        Ok(config) => return config,
        Err(_) => panic!("Could not process the config file."),
    }
}

/// Checks whether user row exists in the database. Creates one if it doesn't exist.
fn check_user(connection: &Connection, user_id: &str, username: &str) {
    connection
        .execute(
            "INSERT OR IGNORE INTO Users (TwitchID, Username) VALUES (?, ?)",
            &[&user_id, &username],
        )
        .unwrap();
}

fn main() {
    let config = load_config();

    let connection = Connection::open(&config.database_path).unwrap();

    let mut client = Client::new();
    let parser = Parser::new(&config.command_prefix);
    let mut modules: Vec<Box<Module>> = Vec::new();

    init_modules(&config, &mut modules);

    client.initialize(&config.oauth, &config.nickname);
    client.join_channels(&config.channels);

    loop {
        if let Ok(line) = client.read_line() {
            match parser.decode(&line) {
                Ok(message) => {
                    match &message {
                        Message::Private(privmsg) => check_user(&connection, &privmsg.tags["user-id"], &privmsg.tags["display-name"]),
                        Message::Command(command) => check_user(
                            &connection,
                            &command.tags["user-id"],
                            &command.tags["display-name"],
                        ),
                        _ => {}
                    }

                    forward_to_modules(&mut modules, &message, &mut client)
                }
                Err(e) => println!("{}:{}", e, line),
            }
        }
    }
}
