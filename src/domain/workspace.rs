use super::{Stacks, Task};
use crate::strings;

pub struct Workspace {
    pub stacks: Stacks,
}

impl Workspace {
    /// helps find tasks by name
    pub fn task_with_name(&self, name: &str) -> Option<&Task> {
        for stack in &self.stacks {
            if let Some(task) = stack.task_with_name(name) {
                return Some(task);
            }
        }
        None
    }

    pub fn tasks_matching_name(&self, name: &str) -> Vec<&str> {
        let mut task_names = vec![];
        for stack in &self.stacks {
            for task in stack.tasks() {
                task_names.push(task.name.as_str());
            }
        }
        strings::matching(name, task_names)
    }
}
