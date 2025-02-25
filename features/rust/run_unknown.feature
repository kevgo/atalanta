Feature: run an unknown Cargo command

  Scenario: run an unknown task
    Given a file "Cargo.toml" with content:
      """
      [package]
      name = "atalanta"
      version = "0.0.0"
      """
    And a file "Cargo.lock"
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Rust (Cargo)

        build  cargo build
        check  cargo check
        test   cargo test
      """
    Then the exit code is 1
