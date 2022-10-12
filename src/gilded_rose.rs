use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: &str, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.to_string(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Item: (name: {}, sell_in: {}, quality: {})", self.name, self.sell_in, self.quality)
    }
}

#[derive(Debug)]
pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: &[Item]) -> GildedRose {
        GildedRose { items: Vec::from(items) }
    }

    pub fn update_quality(&mut self) {
        let mut i = 0;

        loop {

            if i >= self.items.len() {
                break;
            }

            if !self.items[i].name.eq("Aged Brie")
                && !self.items[i]
                    .name
                    .eq("Backstage passes to a TAFKAL80ETC concert")
            {
                if self.items[i].quality > 0 {
                    if !self.items[i].name.eq("Sulfuras, Hand of Ragnaros") {
                        self.items[i].quality = self.items[i].quality - 1;
                    }
                }
            } else {
                if self.items[i].quality < 50 {
                    self.items[i].quality = self.items[i].quality + 1;

                    if self.items[i]
                        .name
                        .eq("Backstage passes to a TAFKAL80ETC concert")
                    {
                        if self.items[i].sell_in < 11 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }

                        if self.items[i].sell_in < 6 {
                            if self.items[i].quality < 50 {
                                self.items[i].quality = self.items[i].quality + 1;
                            }
                        }
                    }
                }
            }

            if !self.items[i].name.eq("Sulfuras, Hand of Ragnaros") {
                self.items[i].sell_in = self.items[i].sell_in - 1;
            }

            if self.items[i].sell_in < 0 {
                if !self.items[i].name.eq("Aged Brie") {
                    if !self.items[i]
                        .name
                        .eq("Backstage passes to a TAFKAL80ETC concert")
                    {
                        if self.items[i].quality > 0 {
                            if !self.items[i].name.eq("Sulfuras, Hand of Ragnaros") {
                                self.items[i].quality = self.items[i].quality - 1;
                            }
                        }
                    } else {
                        self.items[i].quality = self.items[i].quality - self.items[i].quality;
                    }
                } else {
                    if self.items[i].quality < 50 {
                        self.items[i].quality = self.items[i].quality + 1;
                    }
                }
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{GildedRose, Item};

    #[test]
    pub fn generic_item() {
        let items = &[Item::new("Generic Item", 10, 25)];
        let mut guilded_rose = GildedRose::new(items);

        guilded_rose.update_quality();
    }
}
