pub mod com_bar {
    extern crate termion;
    use termion::raw::{RawTerminal};
    use termion::{clear, cursor};
    use std::io::{Stdout, Write};
    use crate::inputmode::inputmode::InputMode;

    pub struct ComBar {
        y_pos: u16,
    }

    impl ComBar {
        pub fn new(stdout: &mut RawTerminal<Stdout>, y_pos: u16) -> ComBar {
            write!(stdout, 
                   "{}{}{}", 
                   clear::All, 
                   cursor::Goto(1, y_pos), 
                   cursor::Hide).unwrap();
            stdout.flush().unwrap();
            ComBar {y_pos}
        }

        pub fn into_mode(&self, stdout: &mut RawTerminal<Stdout>, mode: InputMode) {
            match mode  {
                InputMode::NoMode => {
                    write!(stdout, "{}", cursor::Hide).unwrap();
                },
                InputMode::ComMode => {
                    write!(stdout, 
                           "{}{}{}:", 
                           cursor::Show,
                           cursor::Goto(1, self.y_pos),
                           clear::CurrentLine).unwrap();
                },
            }
            stdout.flush().unwrap();
        }

        pub fn reset_com(&self, stdout: &mut RawTerminal<Stdout>) {
            write!(stdout, 
                   "{}{}{}", 
                   cursor::Hide,
                   cursor::Goto(1, self.y_pos),
                   clear::CurrentLine).unwrap();
            stdout.flush().unwrap();
        }

        pub fn add_ch(&self, stdout: &mut RawTerminal<Stdout>, ch: char) {
            write!(stdout, 
                 "{}",
                 ch).unwrap();  
            stdout.flush().unwrap();
        }

        pub fn del_ch(&self, stdout: &mut RawTerminal<Stdout>) {
            write!(stdout, 
                 "{}{}",
                 cursor::Left(1),
                 clear::AfterCursor).unwrap();
            stdout.flush().unwrap();
        }

    }
}
