use std::fmt;
use std::str::FromStr;

pub struct Todo {
    pub title: String,
    pub is_done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.is_done { "x" } else { "_" };
        write!(f, "[{}] {}", status, self.title)
    }
}

impl FromStr for Todo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let status = parts.next().unwrap();
        let mut title = parts.next().unwrap_or("").to_string();

        while let Some(part) = parts.next() {
            title.push(' ');
            title.push_str(part);
        }

        match status {
            "[_]" => Ok(Self { title, is_done: false }),
            "[x]" => Ok(Self { title, is_done: true }),
            _ => Err(()),
        }
    }
}
