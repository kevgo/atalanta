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
    Then it prints:
      """
      Task 1
      """

  Scenario: run an unknown task
    When executing "run zonk"
    Then it fails with exit code 1 and the output:
      """
      Task "zonk" does not exist.

      I found these tasks:
      - task1
      - task2
      - failing
      """

  Scenario: a task returns a non-zero exit code
    When executing "run failing"
    Then it fails with exit code 2 and the output:
      """
      running a failing task
      """
