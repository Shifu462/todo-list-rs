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
        println!("What do you want to do?");

        let command = Command::parse(read_line());

        if command == Command::Quit {
            break;
        }

        if command == Command::Unknown {
            println!("Unknown command");
            continue;
        }

        command.apply_to(&mut todo_list);
    }
}
