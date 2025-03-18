use cucumber::gherkin::Step;
use cucumber::{World, given, then, when};
use itertools::Itertools;
use rand::Rng;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::process::{Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, str};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::{fs, io};

#[derive(Debug, World)]
#[world(init = Self::new)]
struct RunWorld {
  /// the directory containing the test files of the current scenario
  dir: PathBuf,

  /// the result of running Atlanta
  output: Option<Output>,
}

impl RunWorld {
  fn new() -> Self {
    Self {
      dir: tmp_dir(),
      output: None,
    }
  }
}

impl RunWorld {
  /// provides the exit code of the Atlanta run
  fn exit_code(&self) -> i32 {
    match &self.output {
      Some(output) => output.status.code().unwrap(),
      None => panic!(),
    }
  }

  /// provides the textual output of the Atlanta run
  fn output(&self) -> Cow<str> {
    match &self.output {
      Some(output) => String::from_utf8_lossy(&output.stdout),
      None => Default::default(),
    }
  }

  /// provides the textual output of the Atlanta run with whitespace trimmed from every line
  fn output_trimmed(&self) -> String {
    self
      .output()
      .trim()
      .lines()
      .map(|line| line.trim_end())
      .join("\n")
  }
}

#[given(expr = "a file {string} with content:")]
async fn a_file_with_content(
  world: &mut RunWorld,
  step: &Step,
  filename: String,
) -> io::Result<()> {
  let content = step.docstring.as_ref().unwrap().trim();
  create_file(&filename, content, &world.dir).await
}

#[given(expr = "a file {string}")]
async fn a_file(world: &mut RunWorld, filename: String) -> io::Result<()> {
  create_file(&filename, "", &world.dir).await
}

#[given(expr = "a folder {string}")]
async fn a_folder(world: &mut RunWorld, name: String) -> io::Result<()> {
  let folder_path = &world.dir.join(name);
  fs::create_dir(folder_path).await
}

#[then(expr = "a globally installed Rust executable {string} exists")]
async fn global_rust_executable_exists(_world: &mut RunWorld, name: String) {
  let executable_path = home::cargo_home().unwrap().join("bin").join(name);
  let metadata = fs::metadata(&executable_path).await.unwrap();
  assert!(metadata.is_file());
  fs::remove_file(executable_path).await.unwrap();
}

#[given("a Makefile with content:")]
async fn a_makefile(world: &mut RunWorld, step: &Step) -> io::Result<()> {
  let content = step.docstring.as_ref().unwrap().trim();
  let tabulized = convert_to_makefile_format(content);
  create_file("Makefile", &tabulized, &world.dir).await
}

#[when(expr = "executing {string}")]
async fn executing(world: &mut RunWorld, command: String) {
  let mut args = command.split_ascii_whitespace();
  let mut cmd = args.next().unwrap();
  if cmd == "a" {
    cmd = "../../target/debug/a";
  }
  world.output = Some(
    Command::new(cmd)
      .args(args)
      .current_dir(&world.dir)
      .output()
      .await
      .expect("cannot find the 'a' executable"),
  );
}

#[when(expr = "executing {string} and pressing the keys:")]
async fn executing_and_pressing_keys(world: &mut RunWorld, command: String) {
  let mut args = command.split_ascii_whitespace();
  let mut cmd = args.next().unwrap();
  if cmd == "a" {
    cmd = "../../target/debug/a";
  }
  let mut cmd = Command::new(cmd)
    .args(args)
    .current_dir(&world.dir)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .kill_on_drop(true) // kill the process if the test suite terminates
    .spawn()
    .unwrap();
  // This code works with normal subshell commands like "cat", but not with the atalanta executable.
  // The Atalanta executable ignores all input it receives programmatically via STDIN, and always reads the physical keyboard.
  // Maybe Atalanta's terminal library (crossterm) performs low-level API calls to the OS to read keyboard input?
  if let Some(mut stdin) = cmd.stdin.take() {
    stdin.write_all(b"j\n").await.unwrap();
    stdin.flush().await.unwrap();
    stdin.shutdown().await.unwrap();
  }
  world.output = Some(
    cmd
      .wait_with_output()
      .await
      .expect("cannot find the 'a' executable"),
  );
}

#[when(expr = "executing {string} in the {string} folder")]
async fn when_executing_in_folder(world: &mut RunWorld, command: String, folder: String) {
  let mut args = command.split_ascii_whitespace();
  let cmd = args.next().unwrap();
  world.output = Some(
    Command::new(cmd)
      .args(args)
      .current_dir(&world.dir.join(folder))
      .output()
      .await
      .expect("cannot find the 'a' executable"),
  );
}

#[given(expr = "executing {string} in the {string} folder")]
async fn given_executing_in_folder(world: &mut RunWorld, command: String, folder: String) {
  when_executing_in_folder(world, command, folder).await
}

#[given(expr = "I work on the {string} project")]
async fn instantiate_fixture(world: &mut RunWorld, fixture_name: String) {
  let src_path = Path::new("fixtures").join(fixture_name);
  let dst_path = &world.dir;
  copy_dir(src_path, dst_path).await.unwrap()
}

#[then("it prints:")]
fn verify_output(world: &mut RunWorld, step: &Step) {
  let want = step.docstring.as_ref().unwrap().trim();
  let stripped = strip_ansi_escapes::strip(world.output_trimmed());
  let have = str::from_utf8(&stripped).unwrap();
  pretty::assert_eq!(have, want);
}

#[then(expr = "the exit code is {int}")]
fn exit_code(world: &mut RunWorld, want: i32) {
  assert_eq!(world.exit_code(), want);
}

#[then(expr = "the output contains {string}")]
fn output_contains(world: &mut RunWorld, text: String) {
  let output = str::from_utf8(&world.output.as_ref().unwrap().stdout).unwrap();
  if !output.contains(&text) {
    panic!("output does not contain '{text}':\n{output}");
  }
}

#[then(expr = "the workspace contains a folder {string}")]
fn contains_folder(world: &mut RunWorld, folder: String) {
  assert!(world.dir.join(folder).is_dir())
}

/// creates a temporary directory
fn tmp_dir() -> PathBuf {
  let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();
  let rand: String = rand::rng()
    .sample_iter(&rand::distr::Alphanumeric)
    .take(3)
    .map(char::from)
    .collect();
  let cwd = env::current_dir().expect("cannot determine the current directory");
  let dir = cwd.join("tmp").join(format!("{}-{}", timestamp, rand));
  std::fs::create_dir_all(&dir).unwrap();
  dir
}

async fn create_file(filename: &str, content: &str, dir: &Path) -> io::Result<()> {
  let filepath = dir.join(filename);
  fs::write(filepath, content.as_bytes()).await
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

async fn copy_dir<S, D>(src: S, dst: D) -> Result<(), std::io::Error>
where
  S: AsRef<Path> + Send + Sync,
  D: AsRef<Path> + Send + Sync,
{
  tokio::fs::create_dir_all(&dst).await?;
  let mut entries = tokio::fs::read_dir(src).await?;
  while let Some(entry) = entries.next_entry().await? {
    let file_type = entry.file_type().await?;
    if file_type.is_dir() {
      copy_dir(entry.path(), dst.as_ref().join(entry.file_name())).await?;
    } else {
      tokio::fs::copy(entry.path(), dst.as_ref().join(entry.file_name())).await?;
    }
  }
  Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
  RunWorld::run("features").await;
}
