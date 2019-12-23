extern crate pancurses;

use pancurses::{initscr, endwin, newwin, A_REVERSE, Window};

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
        return info_bar;
    }

    pub fn print(&self, text: String) {
        self.win.clear();
        self.win.printw(text);
        self.win.refresh();
    }

    pub fn refresh(&self) {
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

    pub fn add_ch(&mut self, ch: &str) {
        self.win.printw(ch);
        self.com.push_str(ch);
    }
}

fn main() {
    let _scr = initscr();
    pancurses::noecho();
    let info_bar = InfoBar::new();
    let mut com_bar = ComBar::new(_scr.get_max_y());
    info_bar.refresh();
    info_bar.print(String::from("Welcome to Viscord!"));
    loop {
        let ch_option = com_bar.win.getch();
        let ch = match ch_option {
            Some(x) => x,
            None => pancurses::Input::Character('a'),
        };
        match ch {
            pancurses::Input::Character(c) => com_bar.add_ch(&c.to_string()),
            _ => (),
        }
    }
    endwin();
}
