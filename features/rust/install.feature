Feature: install a Rust application

  @this
  Scenario Outline:
    Given I ran "cargo init --bin atalanta-install-tester"
    When executing "a <FLAG>" in the "atalanta-install-tester" folder
    Then it prints:
      """
      xxx
      """
    And the exit code is 0
    And I delete the installed Rust executable "atalanta-install-tester"

    Examples:
      | FLAG      |
      | -i        |
      | --install |
