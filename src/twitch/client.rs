use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

pub struct Client {
    reader: BufReader<TcpStream>,
    sender: Sender<String>,
}

impl Client {
    pub fn new(interval: u64) -> Client {
        let stream = TcpStream::connect("irc.chat.twitch.tv:6667").unwrap();

        let (sender, receiver) = channel();

        let mut writer = BufWriter::new(stream.try_clone().unwrap());

        thread::spawn(move || loop {
            match receiver.recv() {
                Ok(message) => {
                    writeln!(writer, "{}\r", message).unwrap();
                    writer.flush().unwrap();
                }
                Err(e) => {
                    error!("{}", e);
                }
            }

            thread::sleep(Duration::from_millis(interval));
        });

        Client {
            reader: BufReader::new(stream.try_clone().unwrap()),
            sender,
        }
    }

    pub fn initialize(&mut self, oauth: &str, _nick: &str) {
        self.send_line(&format!("PASS {}\r\n", oauth));
        self.send_line(&format!("NICK {}\r\n", oauth));
        self.send_line("CAP REQ :twitch.tv/tags\r\n");
    }

    pub fn send_line(&mut self, line: &str) {
        self.sender.send(line.to_string()).unwrap();
    }

    pub fn join_channel(&mut self, channel: &str) {
        self.send_line(&format!("JOIN #{}", channel));
    }

    pub fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();

        self.reader.read_line(&mut line)?;

        Ok(line.trim().to_string())
    }
}
