extern crate bincode;
extern crate chrono;
extern crate rand;
extern crate regex;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate libloading;
#[macro_use]
extern crate lordbornebot_core;

mod data;
mod middleware;
mod modules;
mod twitch;
mod util;

use data::users::check_user;
use libloading::Library;
use lordbornebot_core::{CommandData, Message, Module, PrivateMessage};
use middleware::filter::Filter;
use middleware::Middleware;
use modules::afk::AFK;
use modules::gamble::Gamble;
use modules::load_module;
use modules::points::Points;
use modules::shapes::Shapes;
use rusqlite::Connection;
use std::env::var_os;
use std::ffi::OsString;
use std::fs::File;
use std::path::{Path, PathBuf};
use twitch::client::Client;
use twitch::parser::Parser;
use util::load_json_from_file;

#[derive(Deserialize)]
struct Config {
    oauth: String,
    nickname: String,
    command_prefix: String,
    database_path: PathBuf,
    banphrases_path: PathBuf,
    channels: Vec<String>,
    message_interval: u64,
}

fn forward_to_middlewares(middlewares: &mut Vec<Box<Middleware>>, message: &mut Message) -> bool {
    for middleware in middlewares {
        if !middleware.process_message(message) {
            return false;
        }
    }

    true
}

fn forward_to_modules(modules: &mut Vec<Box<Module>>, message: &Message, client: &mut Client) {
    for module in modules {
        if let Some(out_message) = module.handle_message(&message) {
            match Parser::encode_response(&out_message) {
                Ok(raw_message) => client.send_line(&raw_message),
                Err(e) => println!("{}", e),
            }
        }
    }
}

fn create_db_connection(path: &Path) -> Connection {
    Connection::open(path).unwrap()
}

fn init_modules(
    libraries: &mut Vec<Library>,
    config: &Config,
    _connection: &Connection,
    modules: &mut Vec<Box<Module>>,
) {
    let points_module = Points::new(create_db_connection(&config.database_path));
    let gamble_module = Gamble::new(create_db_connection(&config.database_path));
    let shapes_module = Shapes::new(create_db_connection(&config.database_path));
    //let rpg_module = RPG::new(&config.database_path);
    let afk_module = AFK::new(create_db_connection(&config.database_path));

    modules.push(Box::new(points_module));
    modules.push(Box::new(gamble_module));
    modules.push(Box::new(shapes_module));
    //modules.push(Box::new(rpg_module));
    modules.push(Box::new(afk_module));
}

fn load_banphrases(path: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    Ok(serde_json::from_reader(file).unwrap())
}

fn init_middleware(config: &Config, middlewares: &mut Vec<Box<Middleware>>) {
    // Order matters! :)
    let phrases = load_banphrases(&config.banphrases_path).unwrap();
    let filter_middleware = Filter::new(phrases);

    middlewares.push(Box::new(filter_middleware));
}

fn load_config() -> Config {
    let path = match var_os("BOT_CONFIG_PATH") {
        Some(path) => path,
        None => OsString::from("config.json"),
    };

    match load_json_from_file(path) {
        Ok(config) => config,
        Err(_) => panic!("Could not process the config file."),
    }
}

fn main() {
    env_logger::init();

    let mut libraries = Vec::new();

    let config = load_config();

    let connection = Connection::open(&config.database_path).unwrap();

    let mut client = Client::new(config.message_interval);
    let parser = Parser::new(&config.command_prefix);

    let mut middlewares: Vec<Box<Middleware>> = Vec::new();
    let mut modules: Vec<Box<Module>> = Vec::new();

    init_middleware(&config, &mut middlewares);
    init_modules(&mut libraries, &config, &connection, &mut modules);

    client.initialize(&config.oauth, &config.nickname);
    for channel in &config.channels {
        client.join_channel(&channel);
    }

    loop {
        if let Ok(line) = client.read_line() {
            match parser.decode(&line) {
                Ok(ref mut message) => {
                    if forward_to_middlewares(&mut middlewares, message) {
                        forward_to_modules(&mut modules, &message, &mut client)
                    }

                    match &message {
                        Message::Private(privmsg) | Message::Command(privmsg, _) => {
                            check_user(
                                &connection,
                                &privmsg.tags["user-id"],
                                &privmsg.tags["display-name"],
                            ).unwrap();
                        }
                        Message::Ping => client.send_line("PONG :tmi.twitch.tv"),
                        _ => {}
                    }
                }
                Err(e) => warn!("{}:{}", e, line),
            }
        }
    }
}
