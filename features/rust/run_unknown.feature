Feature: run an unknown Cargo command

  Scenario: run an unknown task
    Given a file "Cargo.toml"
    And a file "Cargo.lock"
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Rust (Cargo)
      """
    Then the exit code is 1
