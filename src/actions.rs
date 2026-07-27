#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum PowerCommand {
    Shutdown,
    Reboot,
    Suspend,
    Exit,
    
    //View
    FocusNext,
    FocusPrev,
    CloseMenu,
}

#[derive(Clone, Copy)]
pub struct CommandSpec {
    pub program: &'static str,
    pub args: &'static [&'static str],
}

#[derive(Clone, Copy)]
pub struct PowerAction {
    pub command: PowerCommand,
    pub label: &'static str,
    pub spec: Option<CommandSpec>,
}

impl CommandSpec {
    pub fn new(program: &'static str, args: &'static [&'static str]) -> Self {
        Self { program, args }
    }
}
