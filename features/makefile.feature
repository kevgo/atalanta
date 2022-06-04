Feature: Makefiles

  Background:
    Given a Makefile with content:
      """
      task1:
        echo "task 1 is running"

      task2:
        echo "task 2 is running"

      failing:
        echo "running a failing task"
        exit 2
      """

  @this
  Scenario: list available tasks
    When executing "run -a"
    Then it prints:
      """
      task1
      task2
      failing
      """

  Scenario: run a task
    When executing "run task1"
    Then the exit code is 0

  Scenario: run an unknown task
    When executing "run zonk"
    Then the exit code is 1

  Scenario: a task returns a non-zero exit code
    When executing "run failing"
    Then the exit code is 2
