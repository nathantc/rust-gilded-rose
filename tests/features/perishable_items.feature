Feature: Perishable Items
    
    Scenario: At the end of each day our system lowers both values for every item
        Given an item
        | name                      | sell_in | quality |
        | +5 Dexterity Vest         | 10      | 20      |
        | Elixir of the Mongoose    | 16      | 17      |
        
        When processing for 5 days
         
        Then item has updated sell_in 
        | name                      | Updated sell_in | Updated quality |
        | +5 Dexterity Vest         | 5               | 15              |
        | Elixir of the Mongoose    | 11              | 12              |

    Scenario: Once the sell by date has passed, `quality` degrades twice as fast
        Given an item
        | name                      | sell_in | quality |
        | +5 Dexterity Vest         | 0       | 20      |
        | Elixir of the Mongoose    | 0       | 17      |
        
        When processing for 5 days
         
        Then item has updated sell_in 
        | name                      | Updated sell_in | Updated quality |
        | +5 Dexterity Vest         | -5              | 10              |
        | Elixir of the Mongoose    | -5              | 7               |

    Scenario: The `quality` of an item is never negative
        Given an item
        | name                      | sell_in | quality |
        | +5 Dexterity Vest         | 5       | 5       |
        | Elixir of the Mongoose    | 5       | 10      |
        
        When processing for 10 days
         
        Then item has updated sell_in 
        | name                      | Updated sell_in | Updated quality |
        | +5 Dexterity Vest         | -5              | 0               |
        | Elixir of the Mongoose    | -5              | 0               |

    Scenario: “Aged Brie” actually increases in `quality` the older it gets
        Given an item
        | name                      | sell_in | quality |
        | Aged Brie                 | 15      | 5       |
        
        When processing for 10 days
         
        Then item has updated sell_in 
        | name                      | Updated sell_in | Updated quality |
        | Aged Brie                 | 5               | 15              |

    Scenario: “Aged Brie” `quality` increases twice as fast after sell by date passes
        Given an item
        | name                      | sell_in | quality |
        | Aged Brie                 | 5       | 5       |
        
        When processing for 10 days
         
        Then item has updated sell_in 
        | name                      | Updated sell_in | Updated quality |
        | Aged Brie                 | -5              | 20              |

    Scenario: The `quality` of an item is never more than 50
        Given an item
        | name                      | sell_in | quality |
        | Aged Brie                 | 5       | 40      |
        
        When processing for 10 days
         
        Then item has updated sell_in 
        | name                      | Updated sell_in | Updated quality |
        | Aged Brie                 | -5              | 50              |

    Scenario: “Sulfuras”, being a legendary item, never has to be sold or decreases in `quality`
        Given an item
        | name                       | sell_in | quality |
        | Sulfuras, Hand of Ragnaros | 5       | 40      |
        
        When processing for 10 days
         
        Then item has updated sell_in 
        | name                       | Updated sell_in | Updated quality |
        | Sulfuras, Hand of Ragnaros |  5              | 40              |
