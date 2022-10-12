#[path = "../src/gilded_rose.rs"]
mod gilded_rose;

use std::convert::Infallible;

use crate::gilded_rose::GildedRose;
use async_trait::async_trait;
use cucumber::{gherkin::Step, given, then, when, World, WorldInit};

#[derive(Debug, WorldInit)]
struct ItemWorld {
    gilded_rose: GildedRose,
}

#[async_trait(?Send)]
impl World for ItemWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            gilded_rose: GildedRose::new(&[]),
        })
    }
}
#[given(expr = "an item")]
fn given_an_item(world: &mut ItemWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            let item = gilded_rose::Item::new(
                &*row[0],
                row[1].parse::<i32>().unwrap(),
                row[2].parse::<i32>().unwrap(),
            );
            world.gilded_rose.items.push(item);
        }
    } else {
        panic!("Expected data table to exist for Items");
    }
}

#[when(expr = "processing for {int} days")]
fn when_processing_for_multiple_days(world: &mut ItemWorld, days: i32) {
    for _day in 1..=days {
        println!("Updating quality!");
        &world.gilded_rose.update_quality();
    }
}

#[then(expr = "item has updated values")]
fn then_item_has_updated_sell_in(world: &mut ItemWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for (i, row) in table.rows.iter().skip(1).enumerate() {
            let item = &world.gilded_rose.items[i];
            let expected_sell_in = row[1].parse::<i32>().unwrap();
            let expected_quality = row[2].parse::<i32>().unwrap();
            assert_eq!(
                expected_sell_in, item.sell_in,
                "Expected Sell-In for item {} is {}, but found {}.",
                item.name, expected_sell_in, item.sell_in
            );
            assert_eq!(
                expected_quality, item.quality,
                "Expected Quality for item {} is {}, but found {}.",
                item.name, expected_quality, item.quality
            );
        }
    } else {
        panic!("Expected data table to exist for Items");
    }
}

#[tokio::main]
async fn main() {
    ItemWorld::run("tests/features").await;
}
