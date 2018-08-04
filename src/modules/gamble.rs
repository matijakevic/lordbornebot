use rusqlite::Connection;
use twitch::parser::Message;
use modules::Module;

pub struct Gamble {

}

impl Module for Gamble {
    fn handle_message(&self, message: &Message) -> Option<Message> {
        match message {
            Message::Command(command) => {
                if command.name == "roulette" {
                    return Some(privmsg!(&command.channel, "{} lost {} points.", &command.tags["display-name"], 100));
                }
                return None;
            },
            _ => return None
        }
    }
}