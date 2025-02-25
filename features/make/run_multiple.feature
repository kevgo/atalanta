Feature: Makefiles

  Scenario: multiple tasks match a shortcut
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
    When executing "a fo"
    Then it prints:
      """
      Multiple matches:
        format        formats the code
        format-check  checks for formatting problems
      """
    Then the exit code is 1
