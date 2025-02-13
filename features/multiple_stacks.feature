Feature: multiple stacks

  Background:
    Given a Makefile with content:
      """
      task-1:  # task 1 in the Makefile
        echo "make task 1"

      task-2:  # task 2 in the Makefile
        echo "make task 2"
      """
    And a file "package.json" with content:
      """
      {
        "scripts": {
          "task-1": "echo task 1 in package.json",
          "task-2": "echo task 2 in package.json"
        }
      }
      """
    And a file "yarn.lock"
    And a file "Cargo.toml" with content:
      """
      [package]
      name = "foo"
      version = "0.0.0"
      """
    And a file "Cargo.lock"

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Makefile

        task-1  task 1 in the Makefile
        task-2  task 2 in the Makefile

      Node.JS (yarn)

        task-1  echo task 1 in package.json
        task-2  echo task 2 in package.json

      Rust (Cargo)

        build  cargo build
        check  cargo check
        test   cargo test
      """
