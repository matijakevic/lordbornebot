#[macro_use]
extern crate lordbornebot_core;

use lordbornebot_core::{Config, Message, Module};
use std::boxed::Box;

#[no_mangle]
pub extern "C" fn _create_module(config: &Config) -> *mut Module {
    Box::into_raw(Box::new(Ping {}))
}

pub struct Ping {}

impl Module for Ping {
    fn handle_message(&mut self, message: &Message) -> Option<Message> {
        match message {
            Message::Command(privmsg, command) => match command.name.as_ref() {
                "ping" => Some(privmsg!(&privmsg.channel, "pong")),
                "hi" => Some(privmsg!(
                    &privmsg.channel,
                    "Hi from a dynamically loaded module :)"
                )),
                _ => None,
            },
            _ => None,
        }
    }
}
