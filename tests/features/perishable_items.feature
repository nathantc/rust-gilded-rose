@Feature
Feature: Perishable Items

    @Scenario
    Scenario: Daily processing of item
        Given item <name> has a Sell In of <sell_in> days and a Quality of <quality>
        When processing for <n> days
        Then it should perish in <new_sell_in> days
        And it should have a quality of <new_quality>

    @Perishable
    Examples:
        | name                      | n | sell_in | new_sell_in | quality | new_quality |
        | +5 Dexterity Vest         | 5 | 10      | 5           | 20      | 15          | 
        | Elixir of the Mongoose    | 5 | 16      | 11          | 17      | 12          | 

