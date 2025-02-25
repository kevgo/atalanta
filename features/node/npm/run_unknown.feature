Feature: run an unknown Node script

  Scenario: run an unknown task
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
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Node.JS (npm)

        failing
        format
        format:check
      """
    Then the exit code is 1
