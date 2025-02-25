Feature: Makefiles

  Scenario: run an unknown task
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
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Makefile

        format        formats the code
        format-check  checks for formatting problems
        failing       this task returns a non-zero exit code
      """
    Then the exit code is 1
