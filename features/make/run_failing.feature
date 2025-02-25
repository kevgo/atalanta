Feature: building a failing Make target

  Scenario: a task returns a non-zero exit code
    Given a Makefile with content:
      """
      failing:  # this task returns a non-zero exit code
        echo "running a failing task"
        exit 2

      .SILENT:
      """
    When executing "a failing"
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2
