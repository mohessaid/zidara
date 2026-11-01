# 0011. The game is a view of the evidence; live rounds are kernel commands

Date: 2026-11-01. Status: accepted.

## Context
The owner asked for lessons in the style of Duolingo, Brilliant and Kahoot, to show what the
engine can do. Those products keep their own game state: XP, streaks, leaderboards, a quiz
server for live games. Copying that would put a second source of truth beside the evidence log,
let the game claim progress the pedagogy does not see, and need a second sync story. The art
bible already forbids red crosses, losing sounds and countdowns.

## Decision
- **The game layer stores nothing.** XP and streaks are computed from the log on the device;
  stars come from the framework's band. Answer feedback uses the kernel's own marking
  (`check`, which records nothing); the first try at each question is what the quiz submits as
  evidence. The finish screen shows the framework's verdict, so the game never says more than
  the pedagogy does.
- **Pictures and inputs are lesson data (`Look`).** A question can carry a picture, a way to
  answer (type, shade a cut bar, cut and shade) and an explanation. The kernel checks only that
  a look can be drawn and can give the expected answer; experiences decide how it looks.
- **Toys are an activity kind (`Explore`)** that records no evidence.
- **Live rounds are kernel commands**, not a separate service: `StartChallenge`,
  `NextQuestion`, `AnswerChallenge`, `EndChallenge`. Round events have no learner, so every
  phone in the class reads them through the same row-level security as everything else;
  answers are the learner's own. Answers name their round, so a phone whose clock is behind
  cannot put an answer in another round. Ending a round records each learner's answers as
  formative evidence (a game is practice, whatever the quiz is for) through the same rules as
  any quiz. Live rounds are not quiz attempts, so the version rotation is untouched.
- **No timer, no names, no ranking.** The board counts answers and picks per option; the class's
  result is shown together.
- **Phones poll, faster when it matters**: every 4 s while on screen, every 1.5 s during a round,
  never while hidden. No WebSockets: the same command and event endpoints serve everything.

## Consequences
- Phone, browser demo and server agree on every number on screen, offline included, because
  they all read the same log through the same kernel.
- A round reaches phones in about 5 s (measured on the hosted server); inside a round, answers
  and the next question arrive within 1.5 s. A push channel would cut the first delay; it is
  not needed for a class led by a teacher.
- The browser's kernel grew from 164 to 184 KB gzipped (serde's code for the new tagged enums).
- The browser demo can show a live round only to people in different browsers sharing one
  storage, which it does not have; live rounds are shown on the real server.
