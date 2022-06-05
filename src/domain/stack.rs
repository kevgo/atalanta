use super::Task;
use std::fmt::Display;

/// a technology stack that Atalanta knows about
pub trait Stack: Display {
    //
    fn setup(&self) -> Option<(String, Vec<String>)>;

    /// provides all executable tasks for the codebase in the current directory
    fn tasks(&self) -> &Vec<Task>;

    /// provides the task with the given name
    fn task_with_name(&self, name: &str) -> Option<&Task> {
        self.tasks().iter().find(|task| task.name == name)
    }
}

pub type Stacks = Vec<Box<dyn Stack>>;
