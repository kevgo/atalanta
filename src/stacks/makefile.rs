use crate::domain::{Stack, Stacks, Task, Tasks};
use big_s::S;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::fs;
use std::io::ErrorKind;
use std::process::Command;

pub(crate) struct MakefileStack {
  tasks: Tasks,
}

impl Display for MakefileStack {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Makefile")
  }
}

impl Stack for MakefileStack {
  fn setup(&self) -> Option<Command> {
    None
  }

  fn tasks(&self) -> &Tasks {
    &self.tasks
  }
}

pub fn scan(stacks: &mut Stacks) {
  let text = match fs::read_to_string("Makefile") {
    Ok(text) => text,
    Err(e) => match e.kind() {
      ErrorKind::NotFound => return,
      e => {
        println!("Warning: Cannot read file \"Makefile\": {e}");
        return;
      }
    },
  };
  stacks.push(Box::new(MakefileStack {
    tasks: parse_text(&text),
  }));
}

/// provides the tasks in the given Makefile content
fn parse_text(text: &str) -> Tasks {
  let mut result = Tasks::new();
  for line in text.lines() {
    if let Some(task) = parse_line(line) {
      result.push(task);
    }
  }
  result
}

/// provides a task for the Makefile target defined on the given line, if one exists
fn parse_line(line: &str) -> Option<Task> {
  let capture = RE.captures(line)?;
  let name = capture.get(1).unwrap().as_str();
  let desc = match capture.get(4) {
    Some(desc) => desc.as_str().to_string(),
    None => String::new(),
  };
  Some(Task {
    name: name.into(),
    cmd: S("make"),
    argv: vec![S("--no-print-directory"), name.into()],
    desc,
  })
}
static RE: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"^([[[:alnum:]]-]+):([^#]*)?(#[[:blank:]]*(.*))?").unwrap());

#[cfg(test)]
mod tests {

  mod parse_line {
    use crate::domain::Task;
    use big_s::S;

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
        name: S("cuke"),
        cmd: S("make"),
        argv: vec![S("--no-print-directory"), S("cuke")],
        desc: S(""),
      });
      let have = super::super::parse_line(give);
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn name_and_deps() {
      let give = "cuke: build, lint";
      let want = Some(Task {
        name: S("cuke"),
        cmd: S("make"),
        argv: vec![S("--no-print-directory"), S("cuke")],
        desc: S(""),
      });
      let have = super::super::parse_line(give);
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn name_and_desc() {
      let give = "cuke: # run cucumber";
      let want = Some(Task {
        name: S("cuke"),
        cmd: S("make"),
        argv: vec![S("--no-print-directory"), S("cuke")],
        desc: S("run cucumber"),
      });
      let have = super::super::parse_line(give);
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn name_and_deps_and_desc() {
      let give = "cuke: build, lint # run cucumber";
      let want = Some(Task {
        name: S("cuke"),
        cmd: S("make"),
        argv: vec![S("--no-print-directory"), S("cuke")],
        desc: S("run cucumber"),
      });
      let have = super::super::parse_line(give);
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn name_with_dash() {
      let give = "cuke-this:  # run only the tagged Cucumber scenario";
      let want = Some(Task {
        name: S("cuke-this"),
        cmd: S("make"),
        argv: vec![S("--no-print-directory"), S("cuke-this")],
        desc: S("run only the tagged Cucumber scenario"),
      });
      let have = super::super::parse_line(give);
      pretty::assert_eq!(have, want);
    }

    #[test]
    fn name_with_number() {
      let give = "task-1:  # task 1";
      let want = Some(Task {
        name: S("task-1"),
        cmd: S("make"),
        argv: vec![S("--no-print-directory"), S("task-1")],
        desc: S("task 1"),
      });
      let have = super::super::parse_line(give);
      pretty::assert_eq!(have, want);
    }
  }
}
