mod item;
use crate::item::{Item, GildedRose};

fn main() {
    let mut items: Vec<Item> = Vec::new();
    items.push(Item {
        name: "+5 Dexterity Vest".to_string(),
        sell_in: 10,
        quality: 20,
    });
    items.push(Item {
        name: "Aged Brea".to_string(),
        sell_in: 2,
        quality: 0,
    });
    items.push(Item {
        name: "Elixir of the Mongoose".to_string(),
        sell_in: 5,
        quality: 7,
    });
    items.push(Item {
        name: "Sulfuras, Hand of Ragnaros".to_string(),
        sell_in: 0,
        quality: 80,
    });
    items.push(Item {
        name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
        sell_in: 15,
        quality: 20,
    });
    items.push(Item {
        name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
        sell_in: 10,
        quality: 49,
    });
    items.push(Item {
        name: "Backstage passes to a TAFKAL80ETC concert".to_string(),
        sell_in: 5,
        quality: 49,
    });
    items.push(Item {
        name: "Conjured Mana Cake".to_string(),
        sell_in: 3,
        quality: 6,
    });

    let mut gilded_rose = GildedRose { items: items };

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
