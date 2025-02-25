Feature: Node.JS with npm

  Scenario: a task returns a non-zero exit code
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "failing": "echo 'running a failing task' && exit 2"
        }
      }
      """
    And a file "package-lock.json"
    When executing "a failing"
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2
