#[path = "../src/item.rs"]
mod item;
use std::{collections::HashMap, convert::Infallible};

use async_trait::async_trait;
use cucumber::{gherkin::Step, given, then, when, World, WorldInit};

#[derive(Debug, WorldInit)]
struct ItemWorld {
    items: HashMap<String, item::Item>,
}

#[async_trait(?Send)]
impl World for ItemWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            items: HashMap::new(),
        })
    }
}
#[given(expr = "an item")]
fn given_an_item(world: &mut ItemWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            let item = item::Item {
                name: row[0].clone(),
                sell_in: row[1].parse::<i32>().unwrap(),
                quality: row[2].parse::<u32>().unwrap(),
            };
            world.items.insert(row[0].clone(), item);
        }
    } else {
        panic!("Expected data table to exist for Items");
    }
}

#[when(expr = "processing for {int} days")]
fn when_processing_for_multiple_days(world: &mut ItemWorld, days: u32) {
    for (_key, value) in world.items.iter_mut() {
        for _day in 1..=days {
            item::update_quality(value);
        }
    }
}

#[then(expr = "item has updated values")]
fn then_item_has_updated_sell_in(world: &mut ItemWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            let item = world.items.get(&row[0]).unwrap();
            let expected_sell_in = row[1].parse::<i32>().unwrap();
            let expected_quality = row[2].parse::<u32>().unwrap();
            assert_eq!(expected_sell_in, item.sell_in);
            assert_eq!(expected_quality, item.quality);
        }
    } else {
        panic!("Expected data table to exist for Items");
    }
}

#[tokio::main]
async fn main() {
    ItemWorld::run("tests/features").await;
}
