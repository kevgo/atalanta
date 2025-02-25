Feature: run a Node script

  Scenario: run a task
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "format": "echo formatting",
          "format:check": "echo checking format",
          "other": "echo other"
        }
      }
      """
    And a file "package-lock.json"
    When executing "a format"
    Then it prints:
      """
      formatting
      """
    Then the exit code is 0
