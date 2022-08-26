Feature: multiple stacks

  Background:
    Given a file "Cargo.toml"
    And a Makefile with content:
      """
      task-1:  # task 1 in the Makefile
        echo "make task 1"

      task-2:  # task 2 in the Makefile
        echo "make task 2"

      .SILENT:
      """
    And a file "package.json" with content:
      """
      {
        "name": "demo",
        "scripts": {
          "task-1": "echo task 1 in package.json",
          "task-2": "echo task 2 in package.json"
        }
      }
      """
    And a file "yarn.lock"

  @this
  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Makefile

        mtask-1  task 2 in the Makefile
        mtask-2  task 1 in the Makefile

      Node.JS (yarn)

        ntask-1   task 1 in package.json
        ntask-2   task 2 in package.json

      Rust (Cargo)

        build
        check
        test
      """
