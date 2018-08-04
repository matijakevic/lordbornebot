extern crate chrono;
extern crate rand;
extern crate regex;
extern crate rusqlite;

extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod twitch;
use twitch::client::Client;
use twitch::parser::{Message, Parser};
use std::fs::File;
use std::env::var_os;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
	oauth: String,
	nickname: String,
	command_prefix: String,
	channels: Vec<String>
}

fn load_config(path: &Path) -> Config {
	let file = File::open(path).unwrap();
	serde_json::from_reader(file).unwrap()
}

fn main() {
	let config = match var_os("BOT_CONFIG_PATH") {
		Some(path) => load_config(Path::new(&path)),
		None => load_config(Path::new("config.json"))
	};

	let mut client = Client::new();
	client.initialize(&config.oauth, &config.nickname);

	for channel in config.channels {
		client.join_channel(&channel);
	}

	let parser = Parser::new(&config.command_prefix);

	loop {
		match client.read_line() {
			Ok(line) => {
				match parser.decode(&line) {
					Ok(message) => match message {
						Message::Private(privmsg) => println!(
							"{}@{}: {}",
							privmsg.tags.unwrap()["display-name"],
							privmsg.channel,
							privmsg.text
						),
						Message::Command(command) => match command.name.as_ref() {
							"ping" => client.private_message(&command.channel, "pong"),
							"hi" => client.private_message(&command.channel, &format!("Hi, {}! :)", command.tags["display-name"])),
							_ => {}
						},
						Message::Unknown => {}
					},
					Err(e) => println!("{}:{}", e, line)
				}
			}
			Err(e) => println!("Could not read a line from the socket {}", e),
		}
	}
}
