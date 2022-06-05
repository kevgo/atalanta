use std::fmt::Display;
use std::process::{ExitCode, Termination};

mod commands;
mod probes;

/// all commands that could be run
enum Command {
    List,
    Run(String),
}

fn parse_cli_args<AS: AsRef<str>>(mut args: impl Iterator<Item = AS>) -> Command {
    match args.next() {
        Some(cmd) => Command::Run(cmd.as_ref().into()),
        None => Command::List,
    }
}

/// a technology stack that Atalanta knows about
pub trait Stack: Display {
    /// provides all executable tasks for the codebase in the current directory
    fn tasks(&self) -> Vec<Task>;

    /// provides the task with the given name
    fn task_with_name(&self, name: &str) -> Option<Task> {
        self.tasks().into_iter().find(|task| task.name == name)
    }
}

type Stacks = Vec<Box<dyn Stack>>;

pub struct Workspace {
    stacks: Stacks,
}

impl Workspace {
    fn task_with_name(&self, name: &str) -> Option<Task> {
        for stack in &self.stacks {
            if let Some(task) = stack.task_with_name(name) {
                return Some(task);
            }
        }
        None
    }
}

/// a task that can be executed
#[derive(Debug, PartialEq)]
pub struct Task {
    /// name of this task, for running it
    pub name: String,
    /// the command to run
    pub cmd: String,
    /// optional description
    pub desc: Option<String>,
}

/// end result of an Atalanta run
pub enum Outcome {
    /// successfully executed the requested command
    CommandExecuted {
        /// the exit code of the executed command
        exit_code: u8,
    },
    /// Atalanta command ran successfully
    Ok,
    /// couldn't determine a stack
    UnknownStack,
    /// the task with the given name is unknown
    UnknownTask(String, Workspace),
    CannotReadFile {
        path: String,
        error: String,
    },
}

impl Termination for Outcome {
    fn report(self) -> ExitCode {
        match self {
            Outcome::CommandExecuted { exit_code } => ExitCode::from(exit_code),
            Outcome::Ok => ExitCode::SUCCESS,
            Outcome::UnknownStack => {
                println!("Error: cannot determine stack");
                ExitCode::FAILURE
            }
            Outcome::UnknownTask(name, workspace) => {
                println!("Error: task \"{}\" doesn't exist", name);
                commands::list(workspace);
                ExitCode::FAILURE
            }
            Outcome::CannotReadFile { path, error } => {
                println!("Error: cannot read file {}: {}", path, error);
                ExitCode::FAILURE
            }
        }
    }
}

fn main() -> Outcome {
    let stacks = probes::scan();
    if stacks.len() == 0 {
        return Outcome::UnknownStack;
    };
    let workspace = Workspace { stacks };
    match parse_cli_args(std::env::args()) {
        Command::List => commands::list(workspace),
        Command::Run(name) => commands::run(workspace, name),
    }
}
