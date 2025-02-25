Feature: run a Cargo command

  Background:
    Given a file "Cargo.toml" with content:
      """
      [package]
      name = "atalanta"
      version = "0.0.0"
      """
    And a file "Cargo.lock"

  Scenario: run a task via full name
    When executing "a check"
    Then the exit code is 101

  Scenario: run a task via shortcut
    When executing "a c"
    Then the exit code is 101
