pub mod gamble;

use twitch::parser::Message;

pub trait Module {
    fn handle_message(&self, message: &Message) -> Option<Message>;
}