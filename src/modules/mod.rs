pub mod gamble;
pub mod shapes;
pub mod rpg;

use twitch::parser::Message;

pub trait Module {
    fn handle_message(&mut self, message: &Message) -> Option<Message>;
}