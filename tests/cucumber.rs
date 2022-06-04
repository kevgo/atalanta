use async_trait::async_trait;
use camino::Utf8Path;
use cucumber::gherkin::Step;
use cucumber::{given, when, World, WorldInit};
use fs_err as fs;
use fs_err::File;
use rand::Rng;
use std::convert::Infallible;
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, WorldInit)]
pub struct RunWorld {
    /// the directory containing the test files of the current scenario
    pub dir: String,
}

#[async_trait(?Send)]
impl World for RunWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self { dir: tmp_dir() })
    }
}

#[given("a Makefile with content:")]
fn create_makefile(world: &mut RunWorld, step: &Step) {
    let content = step.docstring.as_ref().unwrap().trim();
    let tabulized = convert_to_makefile_format(content);
    create_file("Makefile", &tabulized, &world.dir)
}

#[when(regex = r#"^executing "(.*)"$"#)]
fn executing(world: &mut RunWorld, command: String) {
    println!("1111111");
}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main. Cucumber is composable.
    futures::executor::block_on(RunWorld::run("features"));
}

/// creates a temporary directory
fn tmp_dir() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rand: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(3)
        .map(char::from)
        .collect();
    let dir = format!("./tmp/{}-{}", timestamp, rand);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn create_file<P1: AsRef<Utf8Path>, P2: AsRef<Utf8Path>>(filename: P1, content: &str, dir: P2) {
    let filename = filename.as_ref();
    let dir = dir.as_ref();
    if let Some(parent) = filename.parent() {
        fs::create_dir_all(&dir.join(parent)).unwrap();
    }
    let mut file = File::create(&dir.join(filename)).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

/// this codebase uses 2 spaces for indentation but Makefiles require tabs
fn convert_to_makefile_format(text: &str) -> String {
    let mut result = String::new();
    for line in text.lines() {
        if line.starts_with("  ") {
            result.push('\t');
            result.push_str(&line[2..]);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}
