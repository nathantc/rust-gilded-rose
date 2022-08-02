Feature: Perishable Items

    Scenario: Sell In lowers by one each day
        Given +5 Dexterity Vest sells in 10 days
        Given +5 Dexterity Vest has quaility 20
        When processing for 1 day
        Then +5 Dexterity Vest sells in 9 days
        Then +5 Dexterity Vest has quality 19