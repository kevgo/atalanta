Feature: Makefiles

  Scenario: run an unknown task
    Given a Makefile with content:
      """
      format:  # formats the code
        echo "formatting"

      format-check:  # checks for formatting problems
        echo "check formatting"

      .SILENT:
      """
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Makefile

        format        formats the code
        format-check  checks for formatting problems
      """
    Then the exit code is 1
