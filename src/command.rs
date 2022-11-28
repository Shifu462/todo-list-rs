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

impl Command {
    pub fn parse(input: String) -> Option<Self> {
        let mut parts = input.split_whitespace();

        let command = parts.next().unwrap();

        let mut arg = parts.next().unwrap_or("").to_string();

        while let Some(part) = parts.next() {
            arg.push(' ');
            arg.push_str(part);
        }

        match command {
            "a" | "add" => Some(Self::Add(arg)),
            "t" | "toggle" => Some(Self::Toggle(arg.parse().unwrap())),
            "r" | "remove" => Some(Self::Remove(arg.parse().unwrap())),
            "l" | "list" => Some(Self::List),
            "s" | "save" => Some(Self::Save),
            "q" | "quit" => Some(Self::Quit),
            _ => None,
        }
    }

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
