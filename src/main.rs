mod command;
mod todos;

use command::Command;
use todos::TodoList;

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut todo_list = TodoList
        ::try_restore_saved()
        .unwrap_or(TodoList::new());

    loop {
        println!();
        todo_list.list();

        println!("\nWhat do you want to do?");

        let command = Command::parse(read_line());

        match command {
            Some(Command::Quit) => break,
            Some(command) => command.apply_to(&mut todo_list),
            None => println!("Invalid command"),
        }
    }
}
