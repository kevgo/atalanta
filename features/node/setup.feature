Feature: Node.JS with npm

  Scenario Outline: setup
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "format": "echo formatting",
          "format:check": "echo checking format",
          "failing": "echo 'running a failing task' && exit 2"
        }
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
