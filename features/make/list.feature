Feature: Makefiles

  Background:

  Scenario: list available tasks
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
    When executing "a"
    Then it prints:
      """
      Makefile

        format        formats the code
        format-check  checks for formatting problems
        failing       this task returns a non-zero exit code
      """

  Scenario: no tasks
    Given a Makefile with content:
      """
      .SILENT:
      """
    When executing "a"
    Then it prints:
      """
      Makefile
      """
