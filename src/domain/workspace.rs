use super::{Stacks, Task};

pub struct Workspace {
    pub stacks: Stacks,
}

impl Workspace {
    pub fn task_with_name(&self, name: &str) -> Option<&Task> {
        for stack in &self.stacks {
            if let Some(task) = stack.task_with_name(name) {
                return Some(task);
            }
        }
        None
    }
}
