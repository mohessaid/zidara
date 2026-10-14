Feature: Mastery learning
  Under mastery, a learner moves on when they show the outcome, not when the calendar says.

  Background:
    Given the lesson "equivalent fractions"
    And the class follows "mastery"
    And Amina is a learner
    And Karim is a teacher

  Scenario: Showing mastery opens the next outcome
    When Amina answers quiz-f1 with 2 of 2 right
    Then Amina's outcome F1 is "achieved"
    And Amina's outcome F2 is open

  Scenario: A weak first try is progress, not failure
    When Amina answers quiz-f1 with 0 of 2 right
    Then Amina's outcome F1 is "in progress"
    And Amina's outcome F2 is locked

  Scenario: Two weak tries bring help and tell the teacher
    Given Amina answers quiz-f1 with 2 of 2 right
    When Amina answers quiz-f2 with 1 of 3 right
    And Amina answers quiz-f2 with 1 of 3 right
    Then Amina's outcome F2 is "needs support"
    And Amina is given remedial-f2
    And Karim is told Amina is struggling with F2

  Scenario: Help is given once, however many weak tries
    Given Amina answers quiz-f1 with 2 of 2 right
    When Amina answers quiz-f2 with 0 of 3 right
    And Amina answers quiz-f2 with 0 of 3 right
    And Amina answers quiz-f2 with 0 of 3 right
    Then Amina has been given remedial-f2 exactly once
