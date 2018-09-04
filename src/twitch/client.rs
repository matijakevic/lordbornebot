use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

pub struct Client {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>
}

impl Client {
    pub fn new() -> Client {
        let stream = TcpStream::connect("irc.chat.twitch.tv:6667").unwrap();

        Client {
            reader: BufReader::new(stream.try_clone().unwrap()),
            writer: BufWriter::new(stream.try_clone().unwrap()),
            stream,
        }
    }

    pub fn initialize(&mut self, oauth: &str, nick: &str) {
        write!(&mut self.writer, "PASS {}\r\n", oauth).unwrap();
        write!(&mut self.writer, "NICK {}\r\n", nick).unwrap();
        write!(&mut self.writer, "CAP REQ :twitch.tv/tags\r\n").unwrap();
        self.flush();
    }

    fn flush(&mut self) {
        self.writer.flush().unwrap();
    }

    pub fn join_channel(&mut self, channel: &str) {
        self.send_line(&format!("JOIN #{}", channel));
    }

    pub fn join_channels(&mut self, channels: &Vec<String>) {
        for channel in channels {
            self.send_line_no_flush(&format!("JOIN #{}", channel));
        }

        self.flush();
    }

    pub fn send_line(&mut self, line: &str) {
        self.send_line_no_flush(line);
        self.flush();
    }

    pub fn send_line_no_flush(&mut self, line: &str) {
        write!(self.writer, "{}\r\n", line).unwrap();
    }

    pub fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();

        self.reader.read_line(&mut line)?;

        Ok(line.trim().to_string())
    }
}
