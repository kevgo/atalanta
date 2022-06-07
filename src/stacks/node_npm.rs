use crate::domain::{Stack, Stacks, Task};
use nanoserde::DeJson;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{self};
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

pub struct NodeNpmStack {
    tasks: Vec<Task>,
}

impl Display for NodeNpmStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node.JS (npm)")
    }
}

impl Stack for NodeNpmStack {
    fn setup(&self) -> Option<Command> {
        let mut command = Command::new("npm");
        command.arg("install");
        Some(command)
    }

    fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

#[derive(DeJson)]
pub struct PackageJson {
    pub scripts: HashMap<String, String>,
}

pub fn scan(stacks: &mut Stacks) {
    if !Path::new("package-lock.json").exists() {
        return;
    }
    let package_json = match load_package_json() {
        Some(file) => file,
        None => return,
    };
    stacks.push(Box::new(NodeNpmStack {
        tasks: parse_scripts(package_json),
    }));
}

pub fn load_package_json() -> Option<PackageJson> {
    let file = match fs::read_to_string("package.json") {
        Ok(text) => text,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return None,
            e => {
                println!("Warning: Cannot read file \"package.json\": {}", e);
                return None;
            }
        },
    };
    let package_json: PackageJson = match DeJson::deserialize_json(&file) {
        Ok(content) => content,
        Err(e) => {
            println!(
                "Warning: file \"package.json\" has an invalid structure: {}",
                e
            );
            return None;
        }
    };
    Some(package_json)
}

fn parse_scripts(package_json: PackageJson) -> Vec<Task> {
    let mut result = vec![];
    for (key, value) in package_json.scripts {
        result.push(Task {
            name: key.clone(),
            cmd: "npm".into(),
            argv: vec!["run".into(), key, "--silent".into()],
            desc: value,
        });
    }
    result.sort_unstable_by(Task::sort);
    result
}
