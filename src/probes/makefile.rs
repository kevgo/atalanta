use crate::{Outcome, Stack, Stacks, Task};
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::fs;
use std::io::ErrorKind;

pub struct MakefileStack {
    /// the text of the Makefile
    text: String,
}

pub fn scan(stacks: &mut Stacks) {
    let text = match fs::read_to_string("Makefile") {
        Ok(text) => text,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return,
            e => panic!("Cannot read file \"Makefile\": {}", e),
        },
    };
    stacks.push(Box::new(MakefileStack { text }))
}

impl Display for MakefileStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Makefile")
    }
}

impl Stack for MakefileStack {
    fn tasks(&self) -> Result<Vec<Task>, Outcome> {
        let mut tasks = vec![];
        for line in self.text.lines() {
            if let Some(task) = parse_line(line) {
                tasks.push(task);
            }
        }
        Ok(tasks)
    }
}

/// provides a task for the Makefile target defined on the given line, if one exists
fn parse_line(line: &str) -> Option<Task> {
    let capture = match RE.captures(line) {
        Some(capture) => capture,
        None => return None,
    };
    let name = capture.get(1).unwrap().as_str();
    let desc = match capture.get(4) {
        Some(desc) => desc.as_str().to_string(),
        None => "".to_string(),
    };
    Some(Task {
        name: name.into(),
        cmd: format!("make {}", name),
        desc: Some(desc),
    })
}
static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(\w+):([^#]*)?(#\s*(.*))?"#).unwrap());

#[cfg(test)]
mod tests {

    mod parse_line {
        use crate::Task;

        #[test]
        fn no_task() {
            let give = "\techo hello";
            let want = None;
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name() {
            let give = "cuke:";
            let want = Some(Task {
                name: "cuke".into(),
                cmd: "make cuke".into(),
                desc: Some("".into()),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_and_deps() {
            let give = "cuke: build, lint";
            let want = Some(Task {
                name: "cuke".into(),
                cmd: "make cuke".into(),
                desc: Some("".into()),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_and_desc() {
            let give = "cuke: # run cucumber";
            let want = Some(Task {
                name: "cuke".into(),
                cmd: "make cuke".into(),
                desc: Some("run cucumber".into()),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }

        #[test]
        fn name_and_deps_and_desc() {
            let give = "cuke: build, lint # run cucumber";
            let want = Some(Task {
                name: "cuke".into(),
                cmd: "make cuke".into(),
                desc: Some("run cucumber".into()),
            });
            let have = super::super::parse_line(give);
            pretty::assert_eq!(have, want);
        }
    }
}
