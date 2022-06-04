use std::convert::Infallible;
use async_trait::async_trait;
use cucumber::{given, World, WorldInit};

#[derive(Debug, WorldInit)]
pub struct RunWorld {
    i: i64,
}

// `World` needs to be implemented, so Cucumber knows how to construct it
// for each scenario.
#[async_trait(?Send)]
impl World for RunWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            i: 0,
        })
    }
}

// Steps are defined with `given`, `when` and `then` attributes.
#[given("scenario name")]
fn hungry_cat(world: &mut RunWorld) {
    world.i = 1;
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(RunWorld::run("features"));
}
