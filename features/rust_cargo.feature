Feature: Rust - Cargo

  Background:
    Given a file "Cargo.toml" with content:
      """
      [package]
      name = "atalanta"
      version = "0.0.0"
      """
    And a file "Cargo.lock"

  @this
  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Rust (Cargo)

        build  cargo build
        check  cargo check
        test   cargo test
      """

  Scenario: run a task via full name
    When executing "a check"
    Then it prints:
      """
      building
      """
    Then the exit code is 0

  Scenario: run a task via shortcut
    When executing "a c"
    Then it prints:
      """
      building
      """
    Then the exit code is 0

  Scenario: multiple tasks match a shortcut
    When executing "a e"
    Then it prints:
      """
      Multiple matches:
        format        formats the code
        format-check  checks for formatting problems
      """
    Then the exit code is 1

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Cargo

        build     build
        check     cargo check
        test      test
      """
    Then the exit code is 1

  Scenario: setup
    When executing "a -s"
    Then it prints:
      """
      Warning: I don't know how to set up this workspace
      """
    And the exit code is 1
