Feature: building multiple matching Make targets

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
    When executing "a fo"
    Then it prints:
      """
      Multiple matches:
        format        formats the code
        format-check  checks for formatting problems
      """
    Then the exit code is 1
