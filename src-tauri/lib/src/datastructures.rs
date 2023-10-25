use std::fs::File;
use std::io::prelude::*;

struct Project {
    name: String,
    description: String,
    completed: bool,
    tasks: Vec<Task>,
}

impl Project {
    fn new(name: &str, desc: &str) -> Self {
        Self { name: name.to_owned(), description: desc.to_owned(), completed: false, tasks: Vec::new() }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

pub struct Task {
    name: String,
    description: String,
    completed: bool,
    subtasks: Vec<Subtask>,
}

impl Task {
    pub fn new(name: &str, desc: &str) -> Self {
        Self { name: name.to_owned(), description: desc.to_owned(), completed: false, subtasks: Vec::new() }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn add_subtask(&mut self, subtask: Subtask) {
        self.subtasks.push(subtask);
    }

    pub fn write_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        writeln!(file, "# {}", self.name)?;
        writeln!(file, "{}\n", self.description)?;

        writeln!(file, "### Subtasks")?;
        for subtask in self.subtasks.as_slice() {
            let check = if subtask.completed {"x"} else {" "};
            writeln!(file, "- [{}] {}", check, subtask.description)?;
        }

        for subtask in self.subtasks.as_slice() {
            writeln!(file, "\n### {}", subtask.name)?;
            writeln!(file, "{}", subtask.description)?;
        }
        Ok(())
    }
}

pub struct Subtask {
    name: String,
    description: String,
    completed: bool,
}

impl Subtask {
    pub fn new(name: &str, desc: &str) -> Self {
        Self { name: name.to_owned(), description: desc.to_owned(), completed: false }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}