use async_trait::async_trait;
use cucumber::{given, World, WorldInit};
use fs_err as fs;
use rand::Rng;
use std::convert::Infallible;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, WorldInit)]
pub struct RunWorld {
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
fn create_makefile(_world: &mut RunWorld) {
    fs::write("Makefile", "one").unwrap();
}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main. Cucumber is composable.
    futures::executor::block_on(RunWorld::run("features"));
}

/// creates a temporary directory
pub fn tmp_dir() -> String {
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
