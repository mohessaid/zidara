Feature: Learning offline
  The phone runs the same kernel as the server. Work done offline is replayed on the
  server when the phone reconnects, and both reach the same result.

  Background:
    Given the lesson "equivalent fractions"
    And the class follows "mastery"
    And Amina is a learner
    And Karim is a teacher

  Scenario: Offline work syncs to the same progress
    Given Amina's phone is offline
    When Amina answers quiz-f1 with 2 of 2 right
    And Amina answers quiz-f2 with 1 of 3 right
    And Amina answers quiz-f2 with 1 of 3 right
    And Amina's phone comes back online
    Then the phone and the server agree on Amina's progress
    And Amina is given remedial-f2

  Scenario: A command sent twice counts once
    Given Amina's phone is offline
    When Amina answers quiz-f1 with 2 of 2 right
    And Amina's phone comes back online
    And Amina's phone sends its outbox again
    Then Amina's outcome F1 has 1 piece of evidence
