Feature: cannot set up Makefiles

  Scenario: setup
    Given a Makefile with content:
      """
      .SILENT:
      """
    When executing "a -s"
    Then it prints:
      """
      Warning: cannot set up this stack
      """
    And the exit code is 1
