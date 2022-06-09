Feature: Makefiles

  Background:
    Given a Makefile with content:
      """
      task-1:  # first task
        echo "task 1 is running"

      task-2: task-1  # second task
        echo "task 2 is running"

      failing:  # this task returns a non-zero exit code
        echo "running a failing task"
        exit 2

      .SILENT:
      """

  Scenario: list available tasks
    When executing "a"
    Then it prints:
      """
      Makefile

        task-1   first task
        task-2   second task
        failing  this task returns a non-zero exit code
      """

  Scenario: run a task (full name)
    When executing "a task-1"
    Then it prints:
      """
      task 1 is running
      """
    Then the exit code is 0

  Scenario: run a task (short name)
    When executing "a 1"
    Then it prints:
      """
      task 1 is running
      """
    Then the exit code is 0

  Scenario: multiple tasks match a shortcut
    When executing "a tk"
    Then it prints:
      """
      Multiple matches:
        task-1  first task
        task-2  second task
      """
    Then the exit code is 1

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Makefile

        task-1   first task
        task-2   second task
        failing  this task returns a non-zero exit code
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
    Then it prints:
      """
      Warning: I don't know how to set up this workspace
      """
    And the exit code is 1
