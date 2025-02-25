Feature: listing all available Node scripts

  Scenario: list available tasks
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "format": "echo formatting",
          "format:check": "echo checking format",
          "another": "echo another script"
        }
      }
      """
    And a file "package-lock.json"
    When executing "a"
    Then it prints:
      """
      Node.JS (npm)

        another
        format
        format:check
      """

  Scenario: no tasks
    Given a file "package.json" with content:
      """
      {
        "name": "demo"
      }
      """
    And a file "yarn.lock"
    When executing "a"
    Then it prints:
      """
      Node.JS (yarn)
      """
