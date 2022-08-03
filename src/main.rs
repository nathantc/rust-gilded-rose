mod item;
use crate::item::{Item, GildedRose};

fn main() {

    let mut gilded_rose = GildedRose::new();

    gilded_rose.add_item(Item {
        name: "+5 Dexterity Vest".to_string(),
        sell_in: 10,
        quality: 20,
    });
    gilded_rose.add_item(Item {
        name: "Aged Brea".to_string(),
        sell_in: 2,
        quality: 0,
    });
    gilded_rose.add_item(Item {
        name: "Elixir of the Mongoose".to_string(),
        sell_in: 5,
        quality: 7,
    });
    gilded_rose.add_item(Item {
        name: "Sulfuras, Hand of Ragnaros".to_string(),
        sell_in: 0,
        quality: 80,
    });
    gilded_rose.add_item(Item {
        name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
        sell_in: 15,
        quality: 20,
    });
    gilded_rose.add_item(Item {
        name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
        sell_in: 10,
        quality: 49,
    });
    gilded_rose.add_item(Item {
        name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
        sell_in: 5,
        quality: 49,
    });
    gilded_rose.add_item(Item {
        name: "Conjured Mana Cake".to_string(),
        sell_in: 3,
        quality: 6,
    });

    println!(":::: Gilded Rose Inventory ::::");
    println!("");
    let mut days = 0;
    'daily_update: loop {
        if days >= 5 {
            break 'daily_update;
        }

        days += 1;

        gilded_rose.update_quality();

        let item_iter = gilded_rose.items.iter();

        println!("Day {}", days);
        print!("{}\r", "Name");
        println!("\t\t\t\t\t\t{}\t{}", "Sell In", "Quality");

        for item in item_iter {
            print!("{}\r", item.name);
            println!("\t\t\t\t\t\t{}\t{}", item.sell_in, item.quality);
        }

        println!("");
    }
}
