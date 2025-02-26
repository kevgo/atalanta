Feature: set up a Rust codebase

  Scenario: setup
    Given a file "Cargo.toml"
    And a file "Cargo.lock"
    When executing "a -s"
    Then it prints:
      """
      Warning: cannot set up this stack
      """
    And the exit code is 1
