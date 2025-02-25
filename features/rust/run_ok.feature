Feature: run a Cargo command

  Background:
    Given a file "Cargo.toml"
    And a file "Cargo.lock"

  Scenario: run a task via full name
    When executing "a check"
    Then the exit code is 101

  Scenario: run a task via shortcut
    When executing "a c"
    Then the exit code is 101
