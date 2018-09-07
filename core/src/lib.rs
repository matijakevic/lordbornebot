extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::path::PathBuf;

#[macro_export]
macro_rules! privmsg {
    ($channel:expr, $fmt:expr) => ($crate::Message::Private($crate::PrivateMessage {
        tags: ::std::collections::HashMap::new(),
        sender: String::new(),
        text: $fmt.to_string(),
        channel: $channel.to_string()
    }));
    ($channel:expr, $fmt:expr, $($arg:tt)*) => ($crate::Message::Private($crate::PrivateMessage {
        tags: ::std::collections::HashMap::new(),
        sender: String::new(),
        text: format!($fmt, $($arg)*),
        channel: $channel.to_string()
    }));
}

#[macro_export]
macro_rules! whisper {
    ($user:expr, $fmt:expr) => (privmsg!("jtv", concat!("/w {} ", $fmt), $user));
    ($user:expr, $fmt:expr, $($arg:tt)*) => (privmsg!("jtv", concat!("/w {} ", $fmt), $user, $($arg)*));
}

pub enum Message {
    Unknown,
    Private(PrivateMessage),
    Command(PrivateMessage, CommandData),
    Ping,
}

pub struct PrivateMessage {
    pub tags: HashMap<String, String>,
    pub channel: String,
    pub sender: String,
    pub text: String,
}

pub struct CommandData {
    pub name: String,
    pub raw_args: String,
    pub args: Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub oauth: String,
    pub nickname: String,
    pub command_prefix: String,
    pub database_path: PathBuf,
    pub banphrases_path: PathBuf,
    pub modules_path: PathBuf,
    pub channels: Vec<String>,
    pub message_interval: u64,
    pub modules: Vec<String>,
}

pub trait Module {
    fn handle_message(&mut self, message: &Message) -> Option<Message>;
}
