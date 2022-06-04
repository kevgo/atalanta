use std::fmt::Display;

use super::{Stack, Task};

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
            cmd: "one".into(),
            desc: Some("desc".into()),
        }]
    }
}
