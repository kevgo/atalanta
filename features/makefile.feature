Feature: Makefiles

  Background:
    Given a Makefile with content:
      """
      task1:  # first task
        echo "task 1 is running"

      task2: task1  # second task
        echo "task 2 is running"

      failing:  # this task returns a non-zero exit code
        echo "running a failing task"
        exit 2

      .SILENT:
      """

  Scenario: list available tasks
    When executing "atalanta"
    Then it prints:
      """
      Makefile:

      task1    first task
      task2    second task
      failing  this task returns a non-zero exit code
      """

  Scenario: run a task (full name)
    When executing "atalanta task1"
    Then it prints:
      """
      task 1 is running
      """
    Then the exit code is 0

  Scenario: run a task (short name)
    When executing "atalanta 1"
    Then it prints:
      """
      task 1 is running
      """
    Then the exit code is 0

  Scenario: run an unknown task
    When executing "atalanta zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Makefile:

      task1    first task
      task2    second task
      failing  this task returns a non-zero exit code
      """
    Then the exit code is 1

  Scenario: a task returns a non-zero exit code
    When executing "atalanta failing"
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2

  Scenario: setup
    When executing "atalanta -s"
    Then it prints:
      """
      Warning: I don't know how to set up this workspace
      """
    And the exit code is 1
