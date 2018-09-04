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
                    if privmsg.text.contains(banned_phrase) {
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
