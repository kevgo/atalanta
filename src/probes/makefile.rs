use crate::{Outcome, Stack, Stacks, Task};
use std::fmt::Display;
use std::fs;

pub struct MakefileStack {}

pub fn scan(stacks: &mut Stacks) {
    stacks.push(Box::new(MakefileStack {}))
}

impl Display for MakefileStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Makefile")
    }
}

impl Stack for MakefileStack {
    fn tasks(&self) -> Result<Vec<Task>, Outcome> {
        let text = match fs::read_to_string("Makefile") {
            Ok(text) => text,
            Err(e) => {
                return Err(Outcome::CannotReadFile {
                    path: "Makefile".into(),
                    error: e.to_string(),
                })
            }
        };
        Ok(vec![Task {
            name: "task 1".into(),
            cmd: "one".into(),
            desc: Some("desc".into()),
        }])
    }
}
