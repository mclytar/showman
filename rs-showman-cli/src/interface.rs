use std::str::SplitWhitespace;
use std::collections::HashMap;

#[derive(Clone)]
pub enum CommandResult {
    Ok,
    Stop,
    Incomplete,
    NotFound,
    TooManyArguments,
    Unimplemented,
    CustomMessage(String)
}

pub trait InterfaceNode {
    fn parse(&self, _: SplitWhitespace) -> CommandResult;
}

impl InterfaceNode for CommandResult {
    fn parse(&self, _: SplitWhitespace) -> CommandResult {
        self.clone()
    }
}

impl<F> InterfaceNode for F
    where
        F: Fn(SplitWhitespace) -> CommandResult
{
    fn parse(&self, command_list: SplitWhitespace) -> CommandResult {
        self(command_list)
    }
}

pub struct Interface {
    map: HashMap<String, Box<dyn InterfaceNode>>
}

impl Interface {
    pub fn new() -> Interface {
        Interface { map: HashMap::new() }
    }

    pub fn with_configuration<F>(setup: F) -> Interface
        where
            F: Fn(&mut Interface)
    {
        let mut interface = Interface::new();
        setup(&mut interface);
        interface
    }

    pub fn subcommand(&mut self, command: &str, interface: Interface) -> &mut Self {
        self.map.insert(command.to_owned(), Box::new(interface));
        self
    }

    pub fn handle<F>(&mut self, command: &str, handler: F) -> &mut Self
        where
            F: Fn(SplitWhitespace) -> CommandResult + 'static
    {
        self.map.insert(command.to_owned(), Box::new(handler));
        self
    }

    pub fn configure<F>(&mut self, setup: F)
        where
            F: Fn(&mut Interface)
    {
        setup(self);
    }
}

impl InterfaceNode for Interface {
    fn parse(&self, mut command_list: SplitWhitespace) -> CommandResult {
        if let Some(command) = command_list.next() {
            if let Some(handler) = self.map.get(command) {
                handler.parse(command_list)
            } else {
                CommandResult::NotFound
            }
        } else {
            CommandResult::Incomplete
        }
    }
}