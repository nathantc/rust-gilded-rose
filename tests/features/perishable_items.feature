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
