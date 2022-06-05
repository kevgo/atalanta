use crate::{Stack, Stacks, Task};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, ErrorKind};

pub struct NodeNpmStack {
    tasks: Vec<Task>,
}

impl Display for NodeNpmStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node.JS (npm)")
    }
}

impl Stack for NodeNpmStack {
    fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

#[derive(Deserialize)]
struct PackageJson {
    scripts: HashMap<String, String>,
}

pub fn scan(stacks: &mut Stacks) {
    let file = match File::open("package.json") {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return,
            e => {
                println!("Warning: Cannot read file \"package.json\": {}", e);
                return;
            }
        },
    };
    let reader = BufReader::new(file);
    let package_json: PackageJson = match serde_json::from_reader(reader) {
        Ok(content) => content,
        Err(e) => {
            println!(
                "Warning: file \"package.json\" has an invalid structure: {}",
                e
            );
            return;
        }
    };
    stacks.push(Box::new(NodeNpmStack {
        tasks: parse_scripts(package_json),
    }));
}

fn parse_scripts(package_json: PackageJson) -> Vec<Task> {
    let mut result = vec![];
    for (key, value) in package_json.scripts {
        result.push(Task {
            name: key.clone(),
            cmd: "npm".into(),
            argv: vec!["run".into(), key, "--silent".into()],
            desc: Some(value),
        });
    }
    result.sort();
    result
}
