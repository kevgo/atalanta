Feature: Makefiles

  Background:
    Given a Makefile with content:
      """
      format:  # formats the code
        echo "formatting"

      format-check:  # checks for formatting problems
        echo "check formatting"

      failing:  # this task returns a non-zero exit code
        echo "running a failing task"
        exit 2

      .SILENT:
      """

  Scenario: run a task via full name
    When executing "a format"
    Then it prints:
      """
      formatting
      """
    Then the exit code is 0

  Scenario: run a task via shortcut
    When executing "a fc"
    Then it prints:
      """
      check formatting
      """
    Then the exit code is 0
