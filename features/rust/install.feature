Feature: install a Rust application

  @this
  Scenario Outline:
    Given a folder "install-test"
    And executing "cargo init --bin atalanta-install-tester" in the "install-test" folder
    When executing "../../../../target/debug/a <FLAG>" in the "install-test/atalanta-install-tester" folder
    Then the exit code is 0
    And a Rust executable "atalanta-install-tester" exists
    And I delete the installed Rust executable "atalanta-install-tester"

    Examples:
      | FLAG      |
      | -i        |
      | --install |
