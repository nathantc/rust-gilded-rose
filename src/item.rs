#![allow(clippy::all)]

#[derive(Default, Debug)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: u32,
}

pub fn update_quality(item: &mut Item) {
    if !item.name.eq("Aged Brie")
        && !item
            .name
            .eq("Backstage passes to a TAFKAL80ETC concert")
    {
        if item.quality > 0 {
            if !item.name.eq("Sulfuras, Hand of Ragnaros") {
                item.quality = item.quality - 1;
            }
        }
    } else {
        if item.quality < 50 {
            item.quality = item.quality + 1;

            if item
                .name
                .eq("Backstage passes to a TAFKAL80ETC concert")
            {
                if item.sell_in < 11 {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }

                if item.sell_in < 6 {
                    if item.quality < 50 {
                        item.quality = item.quality + 1;
                    }
                }
            }
        }
    }

    if !item.name.eq("Sulfuras, Hand of Ragnaros") {
        item.sell_in = item.sell_in - 1;
    }

    if item.sell_in < 0 {
        if !item.name.eq("Aged Brie") {
            if !item
                .name
                .eq("Backstage passes to a TAFKAL80ETC concert")
            {
                if item.quality > 0 {
                    if !item.name.eq("Sulfuras, Hand of Ragnaros") {
                        item.quality = item.quality - 1;
                    }
                }
            } else {
                item.quality = item.quality - item.quality;
            }
        } else {
            if item.quality < 50 {
                item.quality = item.quality + 1;
            }
        }
    }

}
