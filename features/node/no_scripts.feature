Feature: Node.JS with Yarn

  Scenario: list available tasks
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
