Feature: list available Cargo commands

  Scenario: list available tasks
    Given a file "Cargo.toml"
    And a file "Cargo.lock"
    When executing "a"
    Then it prints:
      """
      Rust (Cargo)

        build  cargo build
        check  cargo check
        test   cargo test
      """
