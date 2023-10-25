mod playground;
mod project;

use project::{Task, Subtask};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::Path;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_file() -> std::io::Result<()> {
        playground::write_file()?;
        assert_eq!(playground::read_file()?, "Hello world!");
        std::fs::remove_file("foo.txt")?;
        Ok(())
    }

    #[test]
    fn test_task() -> std::io::Result<()> {
        let mut task = Task::new("Count to 3", "The process of counting to 3");
        task.add_subtask(Subtask::new("Count to 1", "1"));
        task.add_subtask(Subtask::new("Count to 2", "2"));
        task.add_subtask(Subtask::new("Count to 3", "3"));
        task.write_file("task.md")?;
        assert!(Path::new("task.md").exists());
        Ok(())
    }
}
