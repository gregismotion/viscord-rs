use std::thread;
use std::sync::{mpsc};
use std::io::{stdin, stdout, Write};

extern crate termion;
use termion::{
    event::Key,
    raw::{IntoRawMode},
    input::TermRead,
    clear,
    cursor,
    style,
};

mod command;
mod commandmsg;
mod inputmode;
mod com_bar;
mod info_bar;
use command::command::Command;
use commandmsg::commandmsg::CommandMsg; 
use inputmode::inputmode::InputMode;
use com_bar::com_bar::ComBar;
use info_bar::info_bar::InfoBar;

fn quit() {
    std::process::exit(0);
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
        let mut info_bar = InfoBar::new(&mut stdout, x_size);

        for rec in rx_ui {
            match rec {
                CommandMsg::Cleanup => {
                    write!(stdout, "{}{}{}", clear::All, cursor::Show, style::Reset).unwrap();
                    stdout.flush().unwrap();
                }
                CommandMsg::IntoMode(mode) => com_bar.into_mode(&mut stdout, mode),
                CommandMsg::ResetCom => com_bar.reset_com(&mut stdout),
                CommandMsg::AddCh(ch) => com_bar.add_ch(&mut stdout, ch),
                CommandMsg::DelCh => com_bar.del_ch(&mut stdout),
                CommandMsg::ChangeServer(name) => info_bar.change_server(&mut stdout, name),
                CommandMsg::ChangeChannel(name) => info_bar.change_channel(&mut stdout, name),           
                CommandMsg::ChangeTopic(topic) => info_bar.change_topic(&mut stdout, topic),
            }
        }
    });
 
    //DISCORD API
    thread::spawn(move || {
        
    });    

    // COMMAND HELPER
    thread::spawn(move || {      
        let mut com = Command::new();
        
        let exec_com = |com: &str| {
            let com = &com[1..];
            let mut args = com.split_whitespace().collect::<Vec<&str>>();
            let com = args.remove(0);
            match com {
                "q" => {
                    tx_com.send(CommandMsg::Cleanup).unwrap();
                    quit();
                },
                _ => {},
            }   
        };
        let handle_com = |com: &mut Command| {
            match com.mode {
                InputMode::ComMode => {
                    tx_com.send(CommandMsg::ResetCom).unwrap();
                    exec_com(&com.com);
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
