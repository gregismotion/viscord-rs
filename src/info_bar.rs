pub mod info_bar {
    extern crate termion;
    use termion::raw::{RawTerminal};
    use termion::{cursor, style};
    use std::io::{Stdout, Write};

    pub struct InfoBar {
        x_size: u16,
        server: String,
        channel: String,
        topic: String,
    }

    impl InfoBar {
        pub fn new(stdout: &mut RawTerminal<Stdout>, x_size: u16) -> InfoBar {
            let info_bar = InfoBar {x_size: x_size,    
                     server: String::from("Not in server"),
                     channel: String::from("Not in channel"),
                     topic: String::from("No topic"),
            };
            info_bar.refresh_text(stdout);
            info_bar
        }

        fn print(&self, stdout: &mut RawTerminal<Stdout>, text: String) {
            write!(stdout, 
                   "{}{}{}{: ^width$}{}",
                   cursor::Goto(1, 1),
                   style::Invert,
                   text,
                   String::new(),
                   style::Reset,
                   width = usize::from(self.x_size) - text.len()).unwrap();
            stdout.flush().unwrap();
        }

        fn refresh_text(&self, stdout: &mut RawTerminal<Stdout>) {
            self.print(stdout, format!("{0} | {1} | {2}", self.server, self.channel, self.topic));
        }

        pub fn change_server(&mut self, stdout: &mut RawTerminal<Stdout>, name: String) {
           self.server = name;
           self.refresh_text(stdout);
        }

        pub fn change_channel(&mut self, stdout: &mut RawTerminal<Stdout>, name: String) {
           self.channel = name;
           self.refresh_text(stdout);
        }

        pub fn change_topic(&mut self, stdout: &mut RawTerminal<Stdout>, topic: String) {
           self.topic = topic;
           self.refresh_text(stdout);
        }
    }
}
