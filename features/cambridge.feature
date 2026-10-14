Feature: Cambridge-style teaching
  Outcomes open on the scheme of work, and checkpoints are judged against grade boundaries.

  Background:
    Given the lesson "equivalent fractions"
    And the class follows "cambridge-style"
    And Amina is a learner
    And Karim is a teacher

  Scenario: The calendar opens outcomes
    Given it is week 1 of term
    Then Amina's outcome F1 is open
    And Amina's outcome F2 is locked
    When it is week 2 of term
    Then Amina's outcome F2 is open

  Scenario: Practice shows a band but does not judge
    Given it is week 1 of term
    When Amina answers quiz-f1 with 2 of 2 right
    Then Amina's outcome F1 is "in progress"
    And Amina's outcome F1 is in band "cambridge.excelling"

  Scenario: The checkpoint judges against the boundaries
    Given it is week 3 of term
    When Amina answers checkpoint with 3 of 3 right
    Then Amina's outcome F2 is "achieved"

  Scenario: A mark below the expected band flags the learner early
    Given it is week 3 of term
    When Karim records a summative mark of 50% for Amina on F3
    Then Amina's outcome F3 is "needs support"
    And Karim is told Amina is below expected on F3
