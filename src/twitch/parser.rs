use regex::Regex;
use std::collections::HashMap;

pub enum Message {
    Unknown,
    Private(PrivateMessage),
    Command(Command),
}

pub struct PrivateMessage {
    pub tags: Option<HashMap<String, String>>,
    pub channel: String,
    pub sender: Option<String>,
    pub text: String,
}

#[macro_export]
macro_rules! privmsg {
    ($channel:expr, $fmt:expr) => ($crate::twitch::parser::Message::Private(PrivateMessage {
        tags: None,
        sender: None,
        text: $fmt,
        channel: $channel.to_string()
    }));
    ($channel:expr, $fmt:expr, $($arg:tt)*) => ($crate::twitch::parser::Message::Private($crate::twitch::parser::PrivateMessage {
        tags: None,
        sender: None,
        text: format!($fmt, $($arg)*),
        channel: $channel.to_string()
    }));
}

pub struct Command {
    pub tags: HashMap<String, String>,
    pub channel: String,
    pub sender: String,
    pub name: String,
    pub raw_args: Option<String>,
    pub args: Option<Vec<String>>,
}

pub struct Parser<'a> {
    command_prefix: &'a str,
    parse_regex: Regex,
}

impl<'a> Parser<'a> {
    pub fn new(command_prefix: &str) -> Parser {
        Parser {
            command_prefix,
            parse_regex: Regex::new("^(?:@([^ ]*) +)(?::([^ ]+) +)([^ ]+)(?: +([^: ]+[^ ]*(?: +[^: ]+[^ ]*)*))?(?: +:(.*))?[\r\n]*$").unwrap(),
        }
    }

    fn parse_tags(tags: &str) -> HashMap<String, String> {
        let mut map = HashMap::new();

        for tag_pair in tags.split(";") {
            let tag_tokens: Vec<&str> = tag_pair.split("=").collect();
            map.insert(tag_tokens[0].to_string(), tag_tokens[1].to_string());
        }

        return map;
    }

    pub fn decode(&self, line: &str) -> Result<Message, &'static str> {
        if let Some(captures) = self.parse_regex.captures(line) {
            match captures.get(3).unwrap().as_str() {
                "PRIVMSG" => {
                    let text = captures.get(5).unwrap().as_str();
                    let tags = Parser::parse_tags(captures.get(1).unwrap().as_str());
                    let sender = captures.get(2).unwrap().as_str();
                    let channel = captures.get(4).unwrap().as_str().trim_left_matches("#");

                    if text.starts_with(self.command_prefix) {
                        let text = text.trim_left_matches(">>");
                        let tokens: Vec<&str> = text.split(" ").collect();

                        if tokens.len() >= 1 && !tokens[0].is_empty() {
                            let name = tokens[0].to_string();
                            let mut raw_args = None;
                            let mut args: Option<Vec<String>> = None;

                            if tokens.len() > 1 {
                                raw_args = Some((&text[text.find(" ").unwrap()..]).to_string());
                                args = Some(tokens[1..].into_iter().map(|s| s.to_string()).collect());
                            }

                            return Ok(Message::Command(Command {
                                tags,
                                sender: sender.to_string(),
                                channel: channel.to_string(),
                                name,
                                raw_args,
                                args,
                            }));
                        }

                        return Err("No command name provided");
                    }

                    return Ok(Message::Private(PrivateMessage {
                        tags: Some(tags),
                        sender: Some(sender.to_string()),
                        channel: channel.to_string(),
                        text: text.to_string(),
                    }));
                }
                _ => return Ok(Message::Unknown),
            }
        }

        Err("Cannot decode this message")
    }

    pub fn encode(message: &Message) -> Result<String, &'static str> {
        match message {
            Message::Private(privmsg) => {
                return Ok(format!("PRIVMSG #{} :{}", privmsg.channel, privmsg.text));
            }
            _ => return Err("Cannot encode this type of message"),
        }
    }
}
