use std::convert::TryFrom;
use crate::todos::TodoList;

#[derive(PartialEq, Clone, Debug)]
pub enum Command {
    Add(String),
    Toggle(usize),
    Remove(usize),
    List,
    Save,
    Quit,
}

impl TryFrom<String> for Command {
    type Error = ();

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let mut parts = input.split_whitespace();

        let command = parts.next().unwrap();

        let mut arg = parts.next().unwrap_or("").to_string();

        while let Some(part) = parts.next() {
            arg.push(' ');
            arg.push_str(part);
        }

        match command {
            "a" | "add" => Ok(Self::Add(arg)),
            "t" | "toggle" => Ok(Self::Toggle(arg.parse().unwrap())),
            "r" | "remove" => Ok(Self::Remove(arg.parse().unwrap())),
            "l" | "list" => Ok(Self::List),
            "s" | "save" => Ok(Self::Save),
            "q" | "quit" => Ok(Self::Quit),
            _ => Err(()),
        }
    }
}

impl Command {
    pub fn apply_to(self, todo_list: &mut TodoList) {
        match self {
            Command::Add(title) => todo_list.add(title),
            Command::Toggle(index) => todo_list.toggle(index),
            Command::Remove(index) => todo_list.remove(index),
            Command::List => todo_list.list(),
            Command::Save => todo_list.save(),

            Command::Quit => panic!("this command should be handled by the main loop"),
        }
    }
}
