use std::thread;
use std::sync::{mpsc, Arc, Mutex};

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
}

impl ComBar {
    pub fn new(max_y: i32) -> ComBar {
        return ComBar {
            win: newwin(1, 0, max_y - 1, 0),
        }; 
    }

    pub fn reset_com(&mut self) {
        self.win.erase();
    }

    pub fn add_ch(&mut self, ch: String) {
        self.win.printw(ch);
    }

    pub fn del_ch(&mut self) {
        self.win.mv(self.win.get_cur_y(), self.win.get_cur_x() - 1);
        self.win.delch();
    }
}
unsafe impl Send for ComBar {}
unsafe impl Sync for ComBar {}

pub enum CommandMsg {
    AddCh(char),
    DelCh(),
    ResetCom(),
}

fn main() {
    let _scr = initscr();
    let max_y = _scr.get_max_y();
    pancurses::noecho();

    let (tx_input, rx_com) = mpsc::channel::<Input>();
    let (tx_com, rx_curses) = mpsc::channel::<CommandMsg>();
    let (tx_curses, rx_input) = mpsc::channel::<bool>();

    let com_bar = Arc::new(Mutex::new(ComBar::new(max_y)));

    // NCURSES 
    let com_curses = Arc::clone(&com_bar);
    thread::spawn(move || {   
        let mut info_bar = InfoBar::new();
        info_bar.refresh_text();
        for rec in rx_curses {
            let mut com_bar = com_curses.lock().unwrap();
            match rec {
                CommandMsg::ResetCom() => com_bar.reset_com(),
                CommandMsg::AddCh(ch) => com_bar.add_ch(ch.to_string()),
                CommandMsg::DelCh() => com_bar.del_ch(),
            }
            drop(com_bar);
            tx_curses.send(true).unwrap();
        }
    });
    
    // COMMAND HELPER
    thread::spawn(move || {
        let com = String::new();

        let exec_com = |refer: &str| {
            refer.to_string().clear();
            tx_com.send(CommandMsg::ResetCom()).unwrap();
        };
        let add_ch = |ch: char, refer: &str| {
            refer.to_string().push(ch);
            tx_com.send(CommandMsg::AddCh(ch)).unwrap();
        };
        let del_ch = |refer: &str| {
            refer.to_string().pop();
            tx_com.send(CommandMsg::DelCh()).unwrap();
        };

        for input in rx_com {
            match input {
                Input::KeyEnter | Input::Character('\x0A') => exec_com(&com),
                Input::KeyBackspace | Input::Character('\u{7f}') => del_ch(&com),
                Input::Character(ch) => add_ch(ch, &com),
                _ => {},
            }
        }
    });

    // INPUT WATCHER
    let com_input = Arc::clone(&com_bar); 
    let handle = thread::spawn(move || {
        loop {
            let com_bar = com_input.lock().unwrap();
            match com_bar.win.getch() {
                Some(inp) => tx_input.send(inp).unwrap(),
                _ => {},
            }
            drop(com_bar);
            rx_input.recv().unwrap();
        }
    });
    handle.join().unwrap();
}
