Feature: set up a Node stack

  Scenario Outline: setup
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {}
      }
      """
    And a file "package-lock.json"
    When executing "a <COMMAND>"
    Then the output contains "up to date"
    And the exit code is 0

    Examples:
      | COMMAND |
      | -s      |
      | --setup |
