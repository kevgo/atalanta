use super::{Stack, Task};
use std::fmt::Display;

pub struct MakefileStack {}

pub fn scan() -> Option<MakefileStack> {
    Some(MakefileStack {})
}

impl Display for MakefileStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Makefile")
    }
}

impl Stack for MakefileStack {
    fn tasks(&self) -> Vec<Task> {
        vec![Task {
            name: "task 1".into(),
            cmd: "one".into(),
            desc: Some("desc".into()),
        }]
    }
}
