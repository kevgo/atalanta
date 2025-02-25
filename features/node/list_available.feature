Feature: Node.JS with npm

  Scenario: list available tasks
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
    When executing "a"
    Then it prints:
      """
      Node.JS (npm)

        failing
        format
        format:check
      """
