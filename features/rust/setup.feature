Feature: set up a Rust codebase

  Scenario: setup
    Given a file "Cargo.toml" with content:
      """
      [package]
      name = "atalanta"
      version = "0.0.0"
      """
    And a file "Cargo.lock"
    When executing "a -s"
    Then it prints:
      """
      Warning: I don't know how to set up this stack
      """
    And the exit code is 1
