Feature: nested Yarn workspace

  Background:
    Given a file "package.json" with content:
      """
      {
        "workspaces": [
          "tool"
        ],
        "private": true
      }
      """
    And a folder "tool"
    And a file "tool/package.json" with content:
      """
      {
        "name": "tool",
        "scripts": {
          "task-1": "echo one",
          "task-2": "echo two",
          "failing": "echo 'running a failing task' && exit 2"
        }
      }
      """
    And a file "yarn.lock"

  # Scenario: list available tasks
  #   When executing "a" in the "tool" folder
  #   Then it prints:
  #     """
  #     Node.JS (yarn)

  #       failing
  #       task-1
  #       task-2
  #     """

  Scenario: run a task
    When executing "../../../target/debug/a task-1" in the "tool" folder
    Then it prints:
      """
      one
      """
    Then the exit code is 0

  Scenario: run an unknown task
    When executing "../../../target/debug/a zonk" in the "tool" folder
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Node.JS (yarn)

        failing
        task-1
        task-2
      """
    Then the exit code is 1

  Scenario: a task returns a non-zero exit code
    When executing "../../../target/debug/a failing" in the "tool" folder
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2

  Scenario Outline: setup
    When executing "../../../target/debug/a <COMMAND>" in the "tool" folder
    Then the output contains "yarn install"
    And the exit code is 0
    And the workspace contains a folder "node_modules"

    Examples:
      | COMMAND |
      | -s      |
      | --setup |
