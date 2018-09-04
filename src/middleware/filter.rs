use middleware::Middleware;

use twitch::parser::Message;

pub struct Filter {
    banned_phrases: Vec<String>
}

impl Filter {
    pub fn new(banned_phrases: Vec<String>) -> Filter {
        Filter {
            banned_phrases
        }
    }
}

impl Middleware for Filter {
    fn process_message(&mut self, message: &mut Message) -> bool {
        for banned_phrase in &self.banned_phrases {
            match message {
                Message::Private(privmsg) | Message::Command(privmsg, _) => {
                    if privmsg.text.to_lowercase().contains(&banned_phrase.to_lowercase()) {
                        return false
                    }
                },
                _ => {

                }
            }
        }

        true
    }
}
