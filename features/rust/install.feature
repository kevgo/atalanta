Feature: install a Rust application

  @this
  Scenario Outline:
    Given a file "Cargo.toml"
    And a file "Cargo.lock"
    When executing "a <FLAG>"
    Then it prints:
      """
      Warning: I don't know how to set up this stack
      """
    And the exit code is 1

    Examples:
      | FLAG      |
      | -i        |
      | --install |
