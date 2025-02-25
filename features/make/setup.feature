Feature: Makefiles

  Scenario: setup
    Given a Makefile with content:
      """
      .SILENT:
      """
    When executing "a -s"
    Then it prints:
      """
      Warning: I don't know how to set up this stack
      """
    And the exit code is 1
