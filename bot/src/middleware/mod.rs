use lordbornebot_core::Message;

pub mod filter;

pub trait Middleware {
    fn process_message(&mut self, message: &mut Message) -> bool;
}
