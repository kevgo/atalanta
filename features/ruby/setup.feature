Feature: set up a Node stack

  @this
  Scenario Outline: setup
    Given a file "Rakefile" with content:
      """
      require 'bundler'
      require 'bundler/gem_tasks'
      require 'rspec/core/rake_task'

      desc 'Run linter'
      task 'lint' do
      sh 'bundle exec rubocop'
      end
      """
    And a file "Gemfile"
    When executing "a <COMMAND>"
    Then the output contains "up to date"
    And the exit code is 0

    Examples:
      | COMMAND |
      | -s      |
      | --setup |
