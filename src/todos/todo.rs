pub struct Todo {
    pub title: String,
    pub is_done: bool,
}

impl Todo {
    pub fn to_string(&self) -> String {
        let status = if self.is_done { "x" } else { "_" };
        return format!("[{}] {}", status, self.title);
    }

    pub fn from_string(str: String) -> Todo {
        let mut parts = str.split_whitespace();
        let status = parts.next().unwrap();
        let mut title = parts.next().unwrap_or("").to_string();

        while let Some(part) = parts.next() {
            title.push(' ');
            title.push_str(part);
        }

        return Todo {
            title,
            is_done: status == "[x]",
        };
    }
}
