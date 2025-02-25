Feature: Makefiles

  Background:
    Given a Makefile with content:
      """
      .SILENT:
      """

  Scenario: setup
    When executing "a -s"
    Then it prints:
      """
      Warning: I don't know how to set up this stack
      """
    And the exit code is 1
