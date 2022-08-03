#[derive(Default, Debug)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: u32,
}

#[derive(Default, Debug)]
pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new() -> GildedRose {
        GildedRose { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
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
