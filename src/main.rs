use crate::gilded_rose::{GildedRose, Item};

mod gilded_rose;

fn main() {
    let item_vec = vec!(
        Item::new("+5 Dexterity Vest",10, 20),
        Item::new("Aged Brea", 2, 0,),
        Item::new("Elixir of the Mongoose", 5, 7,),
        Item::new("Sulfuras, Hand of Ragnaros", 0, 80,),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20,),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 49,),
        Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 49,),
        Item::new("Conjured Mana Cake", 3, 6,),
    );

    let mut gilded_rose = GildedRose{
        items: item_vec
    };

    println!(":::: Gilded Rose Inventory ::::");
    println!();
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

        println!();
    }
}
