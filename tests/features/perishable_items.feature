Feature: Perishable Items
    
    Scenario: Perishable items degrade daily
        Given an item with sell_in and quality
        | Item                      | sell_in | quality |
        | +5 Dexterity Vest         | 10      | 20      |
        | Elixir of the Mongoose    | 16      | 17      |
        When processing for multiple days
        | Item                      | days |
        | +5 Dexterity Vest         | 5    |
        | Elixir of the Mongoose    | 3    |
        Then item as a updated sell_in and quality
        | Updated Sell In | Updated Quality |
        | 5               | 15              | 
        | 13              | 14              | 
