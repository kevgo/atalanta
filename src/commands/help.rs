use crate::domain::Outcome;

pub(crate) fn help() -> Outcome {
  println!("{}", text());
  Outcome::Success
}

fn text() -> &'static str {
  r#"Atalanta - runs development tasks for a codebase

Usage: "a foo" runs the "foo" task that is defined in your codebase

Flags:
  --setup / -s             run commands to prepare the codebase for development
  --setup-fish-completion  print commands that set up fish completion
  --fish-completion        prints valid completions (used internally by the fish completion logic)
  --help / -h              display help
  --install / -i           install the codebase in executable form on your computer
"#
}
