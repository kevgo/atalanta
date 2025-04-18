Feature: install a Rust application

  Scenario Outline:
    Given a folder "install-test"
    And executing "cargo init --bin atalanta-install-tester" in the "install-test" folder
    When executing "../../../../target/debug/a -i" in the "install-test/atalanta-install-tester" folder
    Then the exit code is 0
    And a globally installed Rust executable "atalanta-install-tester" exists
