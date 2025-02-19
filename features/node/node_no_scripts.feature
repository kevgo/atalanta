Feature: Node.JS with Yarn

  Background:
    Given a file "package.json" with content:
      """
      {
        "name": "demo"
      }
      """
    And a file "yarn.lock"

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Node.JS (yarn)
      """
