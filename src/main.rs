
#![allow(dead_code)]

mod command;
mod todos;

use command::Command;
use todos::TodoList;

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut todo_list = TodoList
        ::try_restore_saved()
        .unwrap_or(TodoList::new());

    loop {
        println!("What do you want to do?");

        let command = Command::parse(read_line());

        if command == Command::Quit {
            break;
        }

        if command == Command::Unknown {
            println!("Unknown command");
            continue;
        }

        todo_list.apply_command(command.to_owned());
    }

    Ok(())
}