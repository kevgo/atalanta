Feature: Rakefile with Bundler

  Background:
    Given a file "Rakefile" with content:
      """
      """
    And a file "Gemfile" with content:
      """
      # frozen_string_literal: true
      source 'https://rubygems.org'
      gem 'diff_match_patch', '~> 0.1.0'
      """
    And a file "Gemfile.lock"

  Scenario: list available tasks
  # run "bundle exec rake --tasks"

  Scenario: run a task
    When executing "a task-1"
    Then it prints:
      """
      one
      """
    And the exit code is 0

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Ruby (Rakefile)

        failing
        task-1
        task-2
      """
    Then the exit code is 1

  Scenario: a task returns a non-zero exit code
    When executing "a failing"
    Then it prints:
      """
      running a failing task
      """
    And the exit code is 2

  Scenario Outline: setup
    When executing "a <COMMAND>"
    Then the output contains "yarn install"
    And the exit code is 0
    And the "bundle exec foo" command works now

    Examples:
      | COMMAND |
      | -s      |
      | --setup |
