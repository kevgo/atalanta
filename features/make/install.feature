Feature: cannot install Makefiles

  @this
  Scenario: setup
    Given a Makefile with content:
      """
      .SILENT:
      """
    When executing "a -i"
    Then it prints:
      """
      Warning: I don't know how to install this stack
      """
    And the exit code is 1
