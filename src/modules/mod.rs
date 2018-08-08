pub mod gamble;
pub mod points;
pub mod rpg;
pub mod shapes;

use twitch::parser::{Message, Response};

pub trait Module {
    fn handle_message(&mut self, message: &Message) -> Option<Response>;
}
