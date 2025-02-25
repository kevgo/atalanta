Feature: list available Cargo commands

  Scenario: list available tasks
    Given a file "Cargo.toml" with content:
      """
      [package]
      name = "atalanta"
      version = "0.0.0"
      """
    And a file "Cargo.lock"
    When executing "a"
    Then it prints:
      """
      Rust (Cargo)

        build  cargo build
        check  cargo check
        test   cargo test
      """
