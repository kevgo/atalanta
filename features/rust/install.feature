Feature: install a Rust application

  @this
  Scenario Outline:
    Given a folder "install_test"
    And executing "cargo init --bin atalanta-install-tester" in the "install-test" folder
    When executing "a <FLAG>" in the "install-test/atalanta-install-tester" folder
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
