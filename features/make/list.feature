Feature: listing all available Make targets

  Scenario: list available tasks
    Given a Makefile with content:
      """
      format:  # formats the code
        echo "formatting"

      format-check:  # checks for formatting problems
        echo "check formatting"

      other:  # another target
        echo "other"

      .SILENT:
      """
    When executing "a"
    Then it prints:
      """
      Makefile

        format        formats the code
        format-check  checks for formatting problems
        other         another target
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
