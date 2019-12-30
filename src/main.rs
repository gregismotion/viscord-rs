use std::thread;
use std::sync::{mpsc};
use std::io::{stdin, stdout, Stdout, Write};

extern crate termion;
use termion::event::Key;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::TermRead;
use termion::{clear, cursor};

fn quit() {
    std::process::exit(0);
}

struct ComBar {
    y_pos: u16,
}

impl ComBar {
    fn new(stdout: &mut RawTerminal<Stdout>, y_pos: u16) -> ComBar {
        write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, y_pos), cursor::Hide);
        stdout.flush().unwrap();
        ComBar {y_pos}
    }

    pub fn into_mode(&self, stdout: &mut RawTerminal<Stdout>, mode: InputMode) {
        match mode  {
            InputMode::NoMode => {
                write!(stdout, "{}", cursor::Hide);
            },
            InputMode::ComMode => {
                write!(stdout, 
                       "{}{}{}:", 
                       cursor::Show,
                       cursor::Goto(1, self.y_pos),
                       clear::CurrentLine);
            },
        }
        stdout.flush().unwrap();
    }

    pub fn reset_com(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, 
               "{}{}{}", 
               cursor::Hide,
               cursor::Goto(1, self.y_pos),
               clear::CurrentLine);
        stdout.flush().unwrap();
    }

    pub fn add_ch(&self, stdout: &mut RawTerminal<Stdout>, ch: char) {
        write!(stdout, 
             "{}",
             ch);  
        stdout.flush().unwrap();
    }

    pub fn del_ch(&self, stdout: &mut RawTerminal<Stdout>) {
        write!(stdout, 
             "{}{}",
             cursor::Left(1),
             clear::AfterCursor);
        stdout.flush().unwrap();
    }

}

struct Command {
    mode: InputMode,
    com: String,
}

impl Command {
    fn new() -> Command {
        Command {
            mode: InputMode::NoMode,
            com: String::new(),
        }
    }

    pub fn change_mode(&mut self, new_mode: InputMode) {
        self.mode = new_mode;
        match self.mode {
            InputMode::ComMode => {
                self.com.push(':');
            },
            _ => {},
        }
    }

    pub fn add_ch(&mut self, ch: char) {
        self.com.push(ch);
    }
    pub fn del_ch(&mut self) {
        self.com.pop();
    }
    pub fn clear(&mut self) {
        self.com.clear();
    }
}

pub enum CommandMsg {
    IntoMode(InputMode),
    AddCh(char),
    DelCh,
    ResetCom,
    ChangeServer(String),
    ChangeChannel(String),
    ChangeTopic(String),
}

pub enum InputMode {
    NoMode,
    ComMode,
}

fn main() {
    let (tx_input, rx_com) = mpsc::channel::<Key>();
    let (tx_com, rx_ui) = mpsc::channel::<CommandMsg>();

    // UI 
    thread::spawn(move || {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let (x_size, y_size) = match termion::terminal_size() {
            Ok(size) => size,
            _ => (0, 0),
        };
        let com_bar = ComBar::new(&mut stdout, y_size);

        for rec in rx_ui {
            match rec {
                CommandMsg::IntoMode(mode) => com_bar.into_mode(&mut stdout, mode),
                CommandMsg::ResetCom => com_bar.reset_com(&mut stdout),
                CommandMsg::AddCh(ch) => com_bar.add_ch(&mut stdout, ch),
                CommandMsg::DelCh => com_bar.del_ch(&mut stdout),
                CommandMsg::ChangeServer(name) => {},
                CommandMsg::ChangeChannel(name) => {},           
                CommandMsg::ChangeTopic(topic) => {},
                _ => {},
            }
        }
    });
    
    // COMMAND HELPER
    thread::spawn(move || {      
        let mut com = Command::new();
        
        let handle_com = |com: &mut Command| {
            match com.mode {
                InputMode::ComMode => {
                    tx_com.send(CommandMsg::ResetCom).unwrap();
                    com.clear();
                    com.change_mode(InputMode::NoMode);
                },
                _ => {}
            }
        };
        let del_ch = |com: &mut Command| {
            match com.mode {
                InputMode::ComMode => {
                    tx_com.send(CommandMsg::DelCh).unwrap();
                    com.del_ch();
                },
                _ => {},
            };
        };
        let add_ch = |com: &mut Command, ch: char| {
            match com.mode {
                InputMode::ComMode => {
                    tx_com.send(CommandMsg::AddCh(ch)).unwrap();
                    com.add_ch(ch);
                },
                _ => {},
            };
        };
        let into_com_mode = |com: &mut Command| {
            match com.mode {
                InputMode::NoMode => {
                    com.clear();
                    tx_com.send(CommandMsg::IntoMode(InputMode::ComMode)).unwrap();
                    com.change_mode(InputMode::ComMode)
                },
                InputMode::ComMode => {
                    add_ch(com, ':');
                }
            }
        };
        let into_no_mode = |com: &mut Command| {
            tx_com.send(CommandMsg::IntoMode(InputMode::NoMode)).unwrap();
            com.change_mode(InputMode::NoMode);
        };

        for input in rx_com {
            match input {
                Key::Char('\n') => handle_com(&mut com),
                Key::Char(':') => into_com_mode(&mut com),
                Key::Esc => into_no_mode(&mut com),
                Key::Backspace => del_ch(&mut com),
                Key::F(1) => quit(),
                Key::Char(ch) => add_ch(&mut com, ch),
                _ => {},
            }
        }
    });

    // INPUT WATCHER
    let handle = thread::spawn(move || {
        let stdin = stdin();
        for key in stdin.keys() {
            match key {
                Ok(key) => tx_input.send(key).unwrap(),
                _ => {},
            }
        }
    });
    handle.join().unwrap();
}
