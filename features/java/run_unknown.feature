Feature: run an unknown Maven command

  Scenario: run an unknown task
    Given a file "pom.xml"
    When executing "a zonk"
    Then it prints:
      """
      Error: task "zonk" doesn't exist
      
      Java (Maven)
      
        validate  check the project is correct
        compile   compile source code
        test      run unit tests
        package   bundle into JAR/WAR
        verify    run integration tests & checks
      """
    Then the exit code is 1
