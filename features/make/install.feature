Feature: cannot install Makefiles

  Scenario Outline:
    Given a Makefile with content:
      """
      .SILENT:
      """
    When executing "a <FLAG>"
    Then it prints:
      """
      Warning: I don't know how to install this stack
      """
    And the exit code is 1

    Examples:
      | FLAG      |
      | -i        |
      | --install |
