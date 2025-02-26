Feature: cannot install Node apps

  Scenario Outline:
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {}
      }
      """
    And a file "package-lock.json"
    When executing "a <COMMAND>"
    Then it prints:
      """
      Warning: cannot install this stack
      """
    And the exit code is 1

    Examples:
      | COMMAND   |
      | -i        |
      | --install |
