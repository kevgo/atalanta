@this
Feature: Eating too much cucumbers may not be good for you

  Scenario: Eating a few isn't a problem
    Given Alice is hungry
    When she eats 4 cucumbers
    Then she is full
