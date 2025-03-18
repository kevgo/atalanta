Feature: cannot install Makefiles

  @this
  Scenario Outline:
    Given I work on the "Makefile" project
    When executing "a <FLAG>"
    Then it prints:
      """
      Warning: cannot install this stack
      """
    And the exit code is 1

    Examples:
      | FLAG      |
      | -i        |
      | --install |
