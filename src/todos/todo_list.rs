use std::{io::{Write, Read}, str::FromStr};

use crate::todos::todo::Todo;

pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { todos: vec![] }
    }

    pub fn try_restore_saved() -> Result<TodoList, ()> {
        let file = std::fs::File::open("todos.txt");

        if file.is_err() {
            return Err(());
        }

        let mut file = file.unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let todos: Vec<Todo> = contents.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| Todo::from_str(&line).unwrap())
            .collect();

        Ok(
            Self { todos }
        )
    }

    pub fn add(&mut self, title: String) {
        self.todos.push(
            Todo {
                title,
                is_done: false,
            }
        );
    }

    pub fn toggle(&mut self, index: usize) {
        if let Some(todo) = self.todos.get_mut(index) {
            todo.is_done = !todo.is_done;
        }
    }

    pub fn remove(&mut self, index: usize) {
        self.todos.remove(index);
    }

    pub fn to_string(&self) -> String {
        self.todos.iter()
            .map(|todo| todo.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn save(&self) {
        let mut file = std::fs::File::create("todos.txt").unwrap();
        file.write_all(self.to_string().as_bytes()).unwrap();
    }

    pub fn list(&self) {
        println!("Your todos:");

        if self.todos.is_empty() {
            println!("-- Empty --");
            return;
        }

        for (i, todo) in self.todos.iter().enumerate() {
            println!("{}. {}", i, todo.to_string());
        }
    }

}
