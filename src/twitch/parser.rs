use regex::Regex;
use std::collections::HashMap;

use lordbornebot_core::{CommandData, Message, PrivateMessage};

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

        for tag_pair in tags.split(';') {
            let tag_tokens: Vec<&str> = tag_pair.split('=').collect();
            map.insert(tag_tokens[0].to_string(), tag_tokens[1].to_string());
        }

        map
    }

    pub fn decode(&self, line: &str) -> Result<Message, &'static str> {
        if line == "PING :tmi.twitch.tv" {
            return Ok(Message::Ping);
        }

        if let Some(captures) = self.parse_regex.captures(line) {
            match captures.get(3).unwrap().as_str() {
                "PRIVMSG" => {
                    let text = captures.get(5).unwrap().as_str();
                    let tags = Parser::parse_tags(captures.get(1).unwrap().as_str());
                    let sender = captures.get(2).unwrap().as_str();
                    let channel = captures.get(4).unwrap().as_str().trim_left_matches('#');

                    if text.starts_with(self.command_prefix) {
                        let text = text.trim_left_matches(self.command_prefix);
                        let tokens: Vec<&str> = text.split(' ').collect();

                        if !tokens.is_empty() && !tokens[0].is_empty() {
                            let name = tokens[0].to_string();
                            let mut raw_args = String::new();

                            let mut args = if tokens.len() > 1 {
                                raw_args = (&text[text.find(' ').unwrap()..]).to_string();
                                tokens[1..].into_iter().map(|s| s.to_string()).collect()
                            } else {
                                Vec::new()
                            };

                            return Ok(Message::Command(
                                PrivateMessage {
                                    tags,
                                    sender: sender.to_string(),
                                    channel: channel.to_string(),
                                    text: text.to_string(),
                                },
                                CommandData {
                                    name,
                                    raw_args,
                                    args,
                                },
                            ));
                        }

                        return Err("No command name provided");
                    }

                    return Ok(Message::Private(PrivateMessage {
                        tags,
                        sender: sender.to_string(),
                        channel: channel.to_string(),
                        text: text.to_string(),
                    }));
                }
                _ => return Ok(Message::Unknown),
            }
        }

        Err("Cannot decode this message")
    }

    pub fn encode_response(message: &Message) -> Result<String, &'static str> {
        Parser::encode(message)
    }

    pub fn encode(message: &Message) -> Result<String, &'static str> {
        match message {
            Message::Private(privmsg) => {
                Ok(format!("PRIVMSG #{} :{}", privmsg.channel, privmsg.text))
            }
            _ => Err("Cannot encode this message"),
        }
    }
}

pub trait Module {
    fn handle_message(&mut self, message: &Message) -> Option<Message>;
}
