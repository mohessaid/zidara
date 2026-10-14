Feature: AI agents act inside rules people approve
  The assessor is an actor with a role. It proposes, the policy decides, teachers keep
  every summative judgement, and everything it does is logged and can be undone.

  Background:
    Given the lesson "equivalent fractions"
    And the class follows "mastery"
    And Amina is a learner
    And Karim is a teacher
    And the assessor agent is on duty

  Scenario: A clear formative answer is marked by the agent
    When Amina explains explain-f2: "You multiply the top and the bottom by the same number"
    And the assessor marks the answer
    Then the proposal is "applied"
    And Amina's outcome F2 has 1 piece of evidence

  Scenario: An unclear answer waits for the teacher
    When Amina explains explain-f2: "it is the same"
    And the assessor marks the answer
    Then the proposal is "queued"
    And Amina's outcome F2 has 0 pieces of evidence
    When Karim accepts the proposal
    Then the proposal is "accepted"
    And Amina's outcome F2 has 1 piece of evidence

  Scenario: Arabic answers are marked the same way
    When Amina explains explain-f2: "نَضْرِبُ البسط والمقام في نفس العدد"
    And the assessor marks the answer
    Then the proposal is "applied"

  Scenario: The teacher can undo what the agent did
    Given Amina explains explain-f2: "You multiply the top and the bottom by the same number"
    And the assessor marks the answer
    When Karim undoes the assessor's mark
    Then Amina's outcome F2 has 0 pieces of evidence

  Scenario: The agent cannot judge summative work
    When assessor tries to record a summative mark of 90% for Amina on F2
    Then it is refused: assessment_agent may not record_summative
