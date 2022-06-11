use crate::domain::Outcome;

pub fn setup() -> Outcome {
    // disable file completions for the entire command
    println!("complete -c a -f");
    // completions for the built-in commands
    println!("complete -c a -a '-s' -d 'set up the codebase'");
    // completions for the tasks in the current directory
    println!("complete -c a -a \"(a --fish-completion)\"");
    Outcome::Success
}

pub fn tasks() -> Outcome {
    println!("task 1\tone");
    println!("task 2\ttwo");
    Outcome::Success
}
