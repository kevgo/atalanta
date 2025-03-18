Feature: Rakefile with Bundler

  Background:
    Given a file "Rakefile" with content:
      """
      require 'bundler'
      require 'bundler/gem_tasks'
      require 'rspec/core/rake_task'

      RSpec::Core::RakeTask.new :spec

      task default: [:lint, :spec]

      desc 'Run linter'
      task 'lint' do
      sh 'bundle exec rubocop'
      end
      """
    And a file "Gemfile"

  # Scenario: list available tasks
  # run "bundle exec rake --tasks"

  Scenario: run a task
    When executing "a lint"
    Then it prints:
      """
      linting
      """
    And the exit code is 0

  Scenario: run an unknown task
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist

      Ruby (Rakefile)

        lint  Run linter
        test  Run tests
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
