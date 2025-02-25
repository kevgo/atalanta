Feature: building multiple matching Make targets

# Scenario: multiple tasks match a shortcut
#   Given a Makefile with content:
#     """
#     format:  # formats the code
#       echo "formatting the code"

#     format-check:  # checks for formatting problems
#       echo "check formatting"

#     other:  # other task
#       echo "other"

#     .SILENT:
#     """
#   When executing "a fo" and pressing the keys:
#     | KEY   | DESCRIPTION              |
#     | enter | select the current entry |
#   Then it prints:
#     """
#     formatting the code
#     """
#   And the exit code is 0
