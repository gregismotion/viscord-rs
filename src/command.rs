pub mod command {
    use crate::inputmode::inputmode::InputMode;

    pub struct Command {
        pub mode: InputMode,
        pub com: String,
    }

    impl Command {
        pub fn new() -> Command {
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
}
