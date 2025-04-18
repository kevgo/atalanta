Feature: building Make targets

  Background:
    Given a Makefile with content:
      """
      format:  # formats the code
        echo "formatting"

      format-check:  # checks for formatting problems
        echo "check formatting"

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
