#[path = "../src/item.rs"] mod item;
use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{World, WorldInit};

#[derive(Debug, WorldInit)]
struct ItemWorld {

}

#[async_trait(?Send)]
impl World for ItemWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {

        })
    }
}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(ItemWorld::run("tests/features"));
}