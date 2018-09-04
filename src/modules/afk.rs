use chrono::{Duration, Utc};
use data::afk::*;
use modules::Module;
use rusqlite::{Connection, Error};
use std::path::Path;
use twitch::parser::{CommandData, Message, PrivateMessage, Response};

pub struct AFK {
    connection: Connection,
}

impl AFK {
    pub fn new(connection: Connection) -> AFK {
        AFK {
            connection,
        }
    }

    fn afk_command(&self, privmsg: &PrivateMessage, command: &CommandData) -> Option<Response> {
        let id = &privmsg.tags["user-id"];

        match get_afk_status(&self.connection, &id) {
            Ok(status) => {
                if status.afk {
                    return None;
                }
            }
            Err(e) => {
                error!("{}", e);
                return None;
            }
        }

        let reason = if command.args.len() >= 1 {
            &command.raw_args
        } else {
            ""
        };

        match set_afk_status(&self.connection, id, reason) {
            Ok(()) => {
                if reason.is_empty() {
                    return Some(Response::Message(privmsg!(
                        &privmsg.channel,
                        "{} is now afk.",
                        &privmsg.tags["display-name"]
                    )));
                } else {
                    return Some(Response::Message(privmsg!(
                        &privmsg.channel,
                        "{} is now afk: {}",
                        &privmsg.tags["display-name"],
                        reason
                    )));
                }
            }
            Err(e) => {
                error!("{}", e);
                return None;
            }
        }
    }

    fn is_afk_command(&self, privmsg: &PrivateMessage, command: &CommandData) -> Option<Response> {
        if command.args.len() >= 1 {
            let username = &command.args[0];

            match get_afk_status_by_username(&self.connection, username) {
                Ok(status) => {
                    if status.afk {
                        let ago = Utc::now() - status.time;

                        let days = ago.num_days();
                        let hours = ago.num_hours() - days * 24;
                        let mins = ago.num_minutes() - days * 24 * 60 - hours * 60;
                        let secs =
                            ago.num_seconds() - days * 24 * 60 * 60 - hours * 60 * 60 - mins * 60;
                        let ago_str = format!("{}d {}h {}m {}s", days, hours, mins, secs);

                        return Some(Response::Message(privmsg!(
                            &privmsg.channel,
                            "{} is afk ({} ago): {}.",
                            username,
                            &ago_str,
                            status.reason
                        )));
                    } else {
                        return Some(Response::Message(privmsg!(
                            &privmsg.channel,
                            "{} is not afk.",
                            username,
                        )));
                    }
                }
                Err(Error::QueryReturnedNoRows) => {
                    return Some(Response::Message(privmsg!(
                        &privmsg.channel,
                        "{} is not afk.",
                        username,
                    )))
                }
                Err(e) => {
                    error!("{}", e);
                    return None;
                }
            }
        }

        return None;
    }

    fn check_if_back(&self, privmsg: &PrivateMessage) -> Option<Response> {
        let id = &privmsg.tags["user-id"];

        match get_afk_status(&self.connection, &id) {
            Ok(status) => {
                if status.afk {
                    let ago = Utc::now() - status.time;

                    let days = ago.num_days();
                    let hours = ago.num_hours() - days * 24;
                    let mins = ago.num_minutes() - days * 24 * 60 - hours * 60;
                    let secs =
                        ago.num_seconds() - days * 24 * 60 * 60 - hours * 60 * 60 - mins * 60;
                    let ago_str = format!("{}d {}h {}m {}s", days, hours, mins, secs);

                    if ago < Duration::seconds(60) {
                        return None;
                    }

                    if let Err(e) = unset_afk_status(&self.connection, id) {
                        error!("{}", e);
                        return None;
                    }

                    if status.reason.is_empty() {
                        return Some(Response::Message(privmsg!(
                            &privmsg.channel,
                            "{} is back ({} ago)!",
                            privmsg.tags["display-name"],
                            &ago_str
                        )));
                    } else {
                        return Some(Response::Message(privmsg!(
                            &privmsg.channel,
                            "{} is back ({} ago): {}",
                            privmsg.tags["display-name"],
                            &ago_str,
                            status.reason
                        )));
                    }
                } else {
                    return None;
                }
            }
            Err(e) => {
                error!("{}", e);
                return None;
            }
        }
    }
}

impl Module for AFK {
    fn handle_message(&mut self, message: &Message) -> Option<Response> {
        match message {
            Message::Command(privmsg, command) => {
                self.check_if_back(&privmsg);
                match command.name.as_ref() {
                    "afk" => self.afk_command(&privmsg, &command),
                    "isafk" => self.is_afk_command(&privmsg, &command),
                    _ => return None,
                }
            }
            Message::Private(privmsg) => self.check_if_back(&privmsg),
            _ => return None,
        }
    }
}
