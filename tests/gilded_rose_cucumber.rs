#[path = "../src/item.rs"]
mod item;
use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::{given, when, then, World, WorldInit};

#[derive(Debug, WorldInit)]
struct ItemWorld {
    gilded_rose: item::GildedRose,
}

#[async_trait(?Send)]
impl World for ItemWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            gilded_rose: item::GildedRose::new(),
        })
    }
}

#[given(expr = "item {string} has a Sell In of {int} days and a Quality of {int}")]
fn given_item_name(world: &mut ItemWorld, name: String, sell_in: i32, quality: u32) {
    world.gilded_rose.add_item(item::Item {name, sell_in, quality});
}

#[when(expr = "processing for {int} days")]
fn when_processing_for_n_days(world: &mut ItemWorld, days: u32) {
    for _ in 1..days {
        world.gilded_rose.update_quality();
    }
}

#[then(expr = "it should perish in {int} days")]
fn then_should_have_sell_in(world: &mut ItemWorld, sell_in: i32) {
    assert_eq!(world.gilded_rose.items[0].sell_in, sell_in);
}

#[then(expr = "it should have a quality of {int}")]
fn then_should_have_quality(world: &mut ItemWorld, quality: u32) {
    assert_eq!(world.gilded_rose.items[0].quality, quality);
}

#[tokio::main]
async fn main() {
    ItemWorld::run("tests/features").await;
}
