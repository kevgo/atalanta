Feature: Node.JS with Yarn

  Background:
    Given a file "package.json" with content:
      """
      {
        "name": "demo"
      }
      """
    And a file "yarn.lock"

  @this
  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Node.JS (yarn)

        (no tasks found)
      """
