Feature: Node.JS with npm

  Background:
    Given a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "task-1": "echo one",
          "task-2": "echo two",
          "failing": "echo 'running a failing task' && exit 2"
        }
      }
      """
    And a file "package-lock.json"

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Node.JS (npm):

      failing  echo 'running a failing task' && exit 2
      task-1   echo one
      task-2   echo two
      """

  Scenario: run a task
    When executing "a task-1"
    Then it prints:
      """
      one
      """
    Then the exit code is 0

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Node.JS (npm):

      failing  echo 'running a failing task' && exit 2
      task-1   echo one
      task-2   echo two
      """
    Then the exit code is 1

  Scenario: a task returns a non-zero exit code
    When executing "a failing"
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2

  Scenario: setup
    When executing "a -s"
    Then the output contains "up to date"
    And the exit code is 0
