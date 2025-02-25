Feature: run multiple Cargo commands

  Scenario: multiple tasks match a shortcut
    Given a file "Cargo.toml" with content:
      """
      [package]
      name = "atalanta"
      version = "0.0.0"
      """
    And a file "Cargo.lock"
    When executing "a e"
    Then it prints:
      """
      Multiple matches:
        check  cargo check
        test   cargo test
      """
    Then the exit code is 1
