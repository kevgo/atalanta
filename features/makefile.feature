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
      """

  Scenario: list available tasks
    When executing "atalanta"
    Then it prints:
      """
      Available commands (Makefile):
      task1    first task
      task2    second task
      failing  this task returns a non-zero exit code
      """

# Scenario: run a task
#   When executing "atalanta task1"
#   Then it prints:
#     """
#     task 1 is running
#     """
#   Then the exit code is 0

# @this
# Scenario: run an unknown task
#   When executing "atalanta zonk"
#   Then the exit code is 1

# Scenario: a task returns a non-zero exit code
#   When executing "atalanta failing"
#   Then the exit code is 2
