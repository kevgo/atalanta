use super::Task;
use std::fmt::Display;
use std::process::Command;

/// a technology stack that Atalanta knows about
pub trait Stack: Display {
    /// provides the command to set up this stack (binary, argv)
    fn setup(&self) -> Option<Command>;

    /// Provides all executable tasks for the codebase in the current directory.
    /// This only emits read references. The stack instance should own the task data.
    fn tasks(&self) -> &Vec<Task>;

    /// provides the task with the given name
    fn task_matching_name(&self, name: &str) -> Option<&Task> {
        self.tasks().iter().find(|task| task.name == name)
    }
}

pub type Stacks = Vec<Box<dyn Stack>>;

#[cfg(test)]
mod tests {

    mod task_matching_name {
        use std::fmt::Display;

        use crate::domain::{Stack, Task};

        struct TestStack {
            tasks: Vec<Task>,
        }

        impl Stack for TestStack {
            fn setup(&self) -> Option<std::process::Command> {
                unimplemented!()
            }
            fn tasks(&self) -> &Vec<Task> {
                unimplemented!()
            }
        }

        impl Display for TestStack {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                unimplemented!()
            }
        }

        #[test]
        fn has_match() {
            let stack = TestStack {
                tasks: vec![
                    Task {
                        name: "task1".into(),
                        ..Task::default()
                    },
                    Task {
                        name: "task2".into(),
                        ..Task::default()
                    },
                ],
            };
            let have = stack.task_matching_name("t1");
        }
    }
}
