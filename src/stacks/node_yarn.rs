use super::node_npm::{load_package_json, PackageJson};
use crate::domain::{Stack, Stacks, Task};
use std::fmt::Display;
use std::path::Path;

pub struct NodeYarnStack {
    tasks: Vec<Task>,
}

impl Display for NodeYarnStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Node.JS (yarn)")
    }
}

impl Stack for NodeYarnStack {
    fn setup(&self) -> Option<(String, Vec<String>)> {
        Some(("yarn".into(), vec!["install".into()]))
    }

    fn tasks(&self) -> &Vec<Task> {
        &self.tasks
    }
}

pub fn scan(stacks: &mut Stacks) {
    if !Path::new("yarn.lock").exists() {
        return;
    }
    let package_json = match load_package_json() {
        Some(file) => file,
        None => return,
    };
    stacks.push(Box::new(NodeYarnStack {
        tasks: parse_scripts(package_json),
    }));
}

fn parse_scripts(package_json: PackageJson) -> Vec<Task> {
    let mut result = vec![];
    for (key, value) in package_json.scripts {
        result.push(Task {
            name: key.clone(),
            cmd: "yarn".into(),
            argv: vec!["--silent".into(), "run".into(), key],
            desc: Some(value),
        });
    }
    result.sort();
    result
}
