mod item;
use crate::item::{update_quality, Item};

fn main() {
    let mut items: Vec<Item> = vec![
        Item {
            name: "+5 Dexterity Vest".to_string(),
            sell_in: 10,
            quality: 20,
        },
        Item {
            name: "Aged Brea".to_string(),
            sell_in: 2,
            quality: 0,
        },
        Item {
            name: "Elixir of the Mongoose".to_string(),
            sell_in: 5,
            quality: 7,
        },
        Item {
            name: "Sulfuras, Hand of Ragnaros".to_string(),
            sell_in: 0,
            quality: 80,
        },
        Item {
            name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
            sell_in: 15,
            quality: 20,
        },
        Item {
            name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
            sell_in: 10,
            quality: 49,
        },
        Item {
            name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
            sell_in: 5,
            quality: 49,
        },
        Item {
            name: "Conjured Mana Cake".to_string(),
            sell_in: 3,
            quality: 6,
        },
    ];


    println!(":::: Gilded Rose Inventory ::::");
    println!();
    let mut days = 0;
    'daily_update: loop {
        if days >= 5 {
            break 'daily_update;
        }

        days += 1;

        println!("Day {}", days);
        print!("Name\r");
        println!("\t\t\t\t\t\tSell In\tQuality");

        for item in items.iter_mut() {
            update_quality(item);

            print!("{}\r", item.name);
            println!("\t\t\t\t\t\t{}\t{}", item.sell_in, &item.quality);
        }

        println!();
    }
}
