pub mod commandmsg {
    use crate::inputmode::inputmode::InputMode;

    pub enum CommandMsg {
        Cleanup,
        IntoMode(InputMode),
        AddCh(char),
        DelCh,
        ResetCom,
        ChangeServer(String),
        ChangeChannel(String),
        ChangeTopic(String),
    }
}
