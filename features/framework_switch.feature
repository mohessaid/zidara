Feature: One kernel, many pedagogies
  The same evidence reads differently under each framework, and switching loses nothing.

  Background:
    Given the lesson "equivalent fractions"
    And the class follows "mastery"
    And Amina is a learner
    And Karim is a teacher
    And it is week 1 of term

  Scenario: Switching the framework redraws the same progress
    Given Amina answers quiz-f1 with 2 of 2 right
    Then Amina's outcome F1 is "achieved"
    And Amina's outcome F2 is open
    When Karim switches the class to "cambridge-style"
    Then Amina's outcome F1 is "in progress"
    And Amina's outcome F2 is locked
    And Amina's outcome F1 has 1 piece of evidence
    When Karim switches the class to "mastery"
    Then Amina's outcome F1 is "achieved"

  Scenario: Only teachers switch frameworks
    When Amina tries to switch the class to "cambridge-style"
    Then it is refused: learner may not change_framework
