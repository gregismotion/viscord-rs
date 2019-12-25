use std::thread;
use std::sync::mpsc;

extern crate pancurses;

use pancurses::{initscr, endwin, newwin, A_REVERSE, Window, Input};

struct InfoBar {
    win: Window,
    server: String,
    channel: String,
    topic: String,
}

impl InfoBar {
    pub fn new() -> InfoBar {
        let info_bar = InfoBar {
            win: newwin(1, 0, 0, 0), 
            server: String::from("Not in server"), 
            channel: String::from("Not in channel"), 
            topic: String::from("No topic"),
        };
        info_bar.win.bkgd(A_REVERSE);
        info_bar.refresh_text();
        return info_bar;
    }

    pub fn print(&self, text: String) {
        self.win.clear();
        self.win.printw(text);
        self.win.refresh();
    }

    pub fn refresh_text(&self) {
       self.print(format!("{0} | {1} | {2}", self.server, self.channel, self.topic));
    }
}

struct ComBar {
    win: Window,
    com: String,
}

impl ComBar {
    pub fn new(max_y: i32) -> ComBar {
        return ComBar {
            win: newwin(1, 0, max_y - 1, 0),
            com: String::new(),
        }; 
    }

    pub fn handle_ch(&mut self, input: Option<Input>) -> Option<&str> {
        match input {
            Some(Input::KeyBackspace) | Some(Input::Character('\u{7f}')) => self.del_ch(),
            Some(Input::Character(c)) => self.add_ch(c),
            _ => {},
        }
        
        match input {
            Some(Input::KeyEnter) | Some(Input::Character('\x0A')) => return Some(&self.com),
            _ => None,
        }

    }

    pub fn reset_com(&mut self) {
        self.com.clear();
        self.win.erase();
    }

    pub fn add_ch(&mut self, ch: char) {
        self.win.printw(ch.to_string());
        self.com.push(ch);
    }

    pub fn del_ch(&mut self) {
        self.win.mv(self.win.get_cur_y(), self.win.get_cur_x() - 1);
        self.win.delch();
        self.com.pop();
    }
}

fn quit() {
    endwin();
    std::process::exit(0);
}

fn main() {
    let _scr = initscr();
    let max_y = _scr.get_max_y();
    pancurses::noecho();

    let (tx_input, rx_com) = mpsc::channel();
    let (tx_com, rx_curses) = mpsc::channel();

    // NCURSES 
    thread::spawn(move || {   
        let mut info_bar = InfoBar::new();
        info_bar.refresh_text();
        for rec in rx_curses {
            info_bar.server = rec;
            info_bar.refresh_text();
        }
    });
    
    // COOMAND HELPER
    thread::spawn(move || {
        for rec in rx_com {
            tx_com.send(rec).unwrap();
        }
    });

    // INPUT WATCHER
    let handle = thread::spawn(move || {
        let mut com_bar = ComBar::new(max_y);
        let exec_com = |com: &str| {
                    tx_input.send(String::from(com)).unwrap();
                    true
        };
        loop {
            let com = match com_bar.handle_ch(com_bar.win.getch()) {
                Some(com) => exec_com(com),
                None => false,
            };
            if com {
                com_bar.reset_com();
            }
        }
    });
    handle.join().unwrap();
}
