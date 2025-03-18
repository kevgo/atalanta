Feature: cannot install Node apps

  Scenario Outline:
    Given a file "Rakefile" with content:
      """
      desc 'Run linter'
      task 'lint' do
      sh 'bundle exec rubocop'
      end
      """
    And a file "Gemfile"
    When executing "a <COMMAND>"
    Then it prints:
      """
      Warning: cannot install this stack
      """
    And the exit code is 1

    Examples:
      | COMMAND   |
      | -i        |
      | --install |
