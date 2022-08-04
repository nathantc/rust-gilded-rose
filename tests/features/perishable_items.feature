Feature: Perishable Items
    
    Scenario: Perishable items sell_in decreases by 1 every day
        Given an item
        | Item                      | sell_in | quality |
        | +5 Dexterity Vest         | 10      | 20      |
        | Elixir of the Mongoose    | 16      | 17      |
        
        When processing for 5 days
        
        Then item has updated sell_in
        | Item                      | Updated Sell In |
        | +5 Dexterity Vest         | 5               |
        | Elixir of the Mongoose    | 11              |
