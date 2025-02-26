Feature: display help

  Scenario Outline:
    When executing "a <FLAG>"
    Then it prints:
      """
      Atalanta - runs development tasks for a codebase

      Usage: "a foo" runs the "foo" task that is defined in your codebase

      Flags:
        --setup / -s             run commands to prepare the codebase
        --setup-fish-completion  print commands that set up fish completion
        --fish-completion        prints valid completions (used internally by the fish completion logic)
        --help / -h              display help
      """
    And the exit code is 0

    Examples:
      | FLAG   |
      | --help |
      | -h     |
