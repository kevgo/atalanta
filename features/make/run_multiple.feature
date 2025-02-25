Feature: building multiple matching Make targets

  @this
  Scenario: multiple tasks match a shortcut
    Given a Makefile with content:
      """
      format:  # formats the code
        echo "formatting"

      format-check:  # checks for formatting problems
        echo "check formatting"

      other:  # other task
        echo "other"

      .SILENT:
      """
    When executing "a fo" and pressing the keys:
      | KEY   | DESCRIPTION              |
      | enter | select the current entry |
    Then it prints:
      """
      formatting
      """
    And the exit code is 1
