//! Runs `features/*.feature` against the kernel and both frameworks.

use std::collections::{BTreeMap, BTreeSet};

use cucumber::{World, given, then, when};
use zidara_frameworks::{Registry, cambridge::CambridgeStyle, mastery::Mastery};
use zidara_kernel::activity::{ActivityKind, Expected, Response, version};
use zidara_kernel::decide::{Command, Env, Envelope, KernelError, decide, next_variant};
use zidara_kernel::event::Event;
use zidara_kernel::evidence::EvidenceKind;
use zidara_kernel::governance::{Policy, assess};
use zidara_kernel::ids::*;
use zidara_kernel::lesson::Lesson;
use zidara_kernel::roles::{Actor, Role};
use zidara_kernel::rules::NotifyReason;
use zidara_kernel::score::Score;
use zidara_kernel::state::fold;
use zidara_kernel::view::{LearnerView, learner_view, studio_view};

/// Monday 5 October 2026, 00:00 UTC: the first day of term in these stories.
const TERM_START: i64 = 1_791_158_400_000;
const WEEK: i64 = 7 * 24 * 60 * 60 * 1000;
const HOUR: i64 = 60 * 60 * 1000;

#[derive(Debug)]
struct Phone {
    log: Vec<Event>,
    ids: SequentialIds,
    outbox: Vec<(Actor, Envelope)>,
}

#[derive(Debug, World)]
#[world(init = Self::new)]
struct Class {
    lesson: Lesson,
    registry: Registry,
    framework: FrameworkId,
    now: Timestamp,
    people: BTreeMap<String, Actor>,
    server: Vec<Event>,
    server_ids: SequentialIds,
    applied: BTreeSet<Uuid>,
    sent: Vec<(Actor, Envelope)>,
    phone: Option<Phone>,
    phone_log: Option<Vec<Event>>,
    commands: u128,
    refused: Option<KernelError>,
}

fn env<'a>(
    lesson: &'a Lesson,
    registry: &'a Registry,
    framework: &'a FrameworkId,
    now: Timestamp,
) -> Env<'a> {
    Env {
        now,
        tenant: TenantId(Uuid::from_u128(1)),
        cohort: CohortId(Uuid::from_u128(2)),
        lesson,
        frameworks: registry,
        default_framework: framework,
        policy: Policy::STANDARD,
    }
}

impl Class {
    fn new() -> Self {
        let opens_at = [("F1", 0), ("F2", 1), ("F3", 2)]
            .into_iter()
            .map(|(o, week)| (OutcomeId::new(o), Timestamp(TERM_START + week * WEEK)))
            .collect();
        Self {
            lesson: serde_json::from_str(include_str!(
                "../../../content/lessons/equivalent-fractions.json"
            ))
            .expect("the demo lesson is valid"),
            registry: Registry::new(vec![
                Box::new(Mastery::standard()),
                Box::new(CambridgeStyle::primary_checkpoint(opens_at)),
            ]),
            framework: FrameworkId::new("mastery"),
            now: Timestamp(TERM_START + HOUR),
            people: BTreeMap::new(),
            server: vec![],
            server_ids: SequentialIds::default(),
            applied: BTreeSet::new(),
            sent: vec![],
            phone: None,
            phone_log: None,
            commands: 0,
            refused: None,
        }
    }

    fn actor(&self, name: &str) -> Actor {
        self.people
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("{name} was not introduced"))
    }

    fn user(&self, name: &str) -> UserId {
        match self.actor(name) {
            Actor::Human { user, .. } => user,
            other => panic!("{name} is not a person: {other:?}"),
        }
    }

    /// Runs a command where the actor is: on the offline phone, or on the server.
    fn try_run(&mut self, name: &str, command: Command) -> Result<(), KernelError> {
        let actor = self.actor(name);
        self.commands += 1;
        let envelope = Envelope {
            id: Uuid::from_u128(self.commands),
            at: self.now,
            command,
        };
        if let Some(mut phone) = self.phone.take() {
            let env = env(&self.lesson, &self.registry, &self.framework, envelope.at);
            let result = decide(
                &actor,
                envelope.command.clone(),
                &phone.log,
                &env,
                &mut phone.ids,
            );
            if let Ok(events) = &result {
                phone.log.extend(events.iter().cloned());
                phone.outbox.push((actor, envelope));
            }
            self.phone = Some(phone);
            return result.map(|_| ());
        }
        self.apply(&actor, envelope)
    }

    fn run(&mut self, name: &str, command: Command) {
        self.try_run(name, command)
            .unwrap_or_else(|e| panic!("{name} was refused: {e}"));
    }

    /// What the server does with an envelope: once per id, at the time the person acted.
    fn apply(&mut self, actor: &Actor, envelope: Envelope) -> Result<(), KernelError> {
        if !self.applied.insert(envelope.id) {
            return Ok(());
        }
        let at = Timestamp(envelope.at.0.min(self.now.0));
        let env = env(&self.lesson, &self.registry, &self.framework, at);
        let events = decide(
            actor,
            envelope.command,
            &self.server,
            &env,
            &mut self.server_ids,
        )?;
        self.server.extend(events);
        Ok(())
    }

    fn view_on(&self, log: &[Event], name: &str) -> LearnerView {
        learner_view(
            self.user(name),
            "en",
            log,
            &env(&self.lesson, &self.registry, &self.framework, self.now),
        )
        .unwrap()
    }

    fn view(&self, name: &str) -> LearnerView {
        self.view_on(&self.server, name)
    }

    fn outcome(&self, name: &str, outcome: &str) -> zidara_kernel::view::OutcomeView {
        self.view(name)
            .outcomes
            .into_iter()
            .find(|o| o.outcome.as_str() == outcome)
            .unwrap()
    }

    /// Answers whichever version of the quiz is due for the learner, `right` of them correctly.
    fn quiz(&self, learner: &str, activity: &str, right: usize, total: usize) -> Command {
        let id = ActivityId::new(activity);
        let Some(ActivityKind::Quiz {
            questions,
            variants,
            ..
        }) = self.lesson.activity(&id).map(|a| &a.kind)
        else {
            panic!("{activity} is not a quiz")
        };
        let history = self.phone.as_ref().map_or(&self.server, |p| &p.log);
        let variant = next_variant(history, self.user(learner), &id, &self.lesson);
        let questions = version(questions, variants, variant).expect("the due version exists");
        assert_eq!(
            questions.len(),
            total,
            "{activity} has {} questions",
            questions.len()
        );
        let responses = questions
            .iter()
            .enumerate()
            .map(|(i, q)| match (&q.expected, i < right) {
                (Expected::Choice { correct, .. }, true) => Response::Choice { index: *correct },
                (Expected::Choice { correct, options }, false) => Response::Choice {
                    index: (correct + 1) % options.len() as u32,
                },
                (Expected::Exactly { num, den }, true) => Response::Fraction {
                    num: *num,
                    den: *den,
                },
                // An equivalent form, so marking is exercised, not just copied.
                (Expected::EquivalentTo { num, den }, true) => Response::Fraction {
                    num: num * 2,
                    den: den * 2,
                },
                (_, false) => Response::Fraction { num: 0, den: 1 },
            })
            .collect();
        Command::SubmitQuiz {
            activity: id,
            variant,
            responses,
        }
    }

    fn latest_proposal(&self) -> (EventId, zidara_kernel::state::ProposalStatus) {
        let state = fold(&self.server);
        let (id, (_, status)) = state
            .proposals
            .iter()
            .next_back()
            .expect("a proposal exists");
        (*id, *status)
    }
}

fn snake(s: &str) -> String {
    s.replace(' ', "_")
}

#[given(expr = "the lesson {string}")]
fn the_lesson(class: &mut Class, title: String) {
    assert_eq!(class.lesson.title.get("en").to_lowercase(), title);
}

#[given(expr = "the class follows {string}")]
fn follows(class: &mut Class, framework: String) {
    class.framework = FrameworkId::new(framework);
}

#[given(expr = "{word} is a learner")]
fn is_learner(class: &mut Class, name: String) {
    let user = UserId(Uuid::from_u128(100 + class.people.len() as u128));
    class.people.insert(
        name,
        Actor::Human {
            user,
            role: Role::Learner,
        },
    );
}

#[given(expr = "{word} is a teacher")]
fn is_teacher(class: &mut Class, name: String) {
    let user = UserId(Uuid::from_u128(100 + class.people.len() as u128));
    class.people.insert(
        name,
        Actor::Human {
            user,
            role: Role::Teacher,
        },
    );
}

#[given("the assessor agent is on duty")]
fn assessor(class: &mut Class) {
    class.people.insert(
        "assessor".into(),
        Actor::Agent {
            agent: AgentId::new("assessor"),
            role: Role::AssessmentAgent,
        },
    );
}

fn set_week(class: &mut Class, week: i64) {
    class.now = Timestamp(TERM_START + (week - 1) * WEEK + HOUR);
}

#[given(expr = "it is week {int} of term")]
fn given_week(class: &mut Class, week: i64) {
    set_week(class, week);
}

#[when(expr = "it is week {int} of term")]
fn when_week(class: &mut Class, week: i64) {
    set_week(class, week);
}

fn answers(class: &mut Class, name: String, activity: String, right: usize, total: usize) {
    let command = class.quiz(&name, &activity, right, total);
    class.run(&name, command);
}

#[given(expr = "{word} answers {word} with {int} of {int} right")]
fn given_answers(class: &mut Class, name: String, activity: String, right: usize, total: usize) {
    answers(class, name, activity, right, total);
}

#[when(expr = "{word} answers {word} with {int} of {int} right")]
fn when_answers(class: &mut Class, name: String, activity: String, right: usize, total: usize) {
    answers(class, name, activity, right, total);
}

fn explains(class: &mut Class, name: String, activity: String, text: String) {
    class.run(
        &name,
        Command::SubmitAnswer {
            activity: ActivityId::new(activity),
            answer: text,
        },
    );
}

#[given(expr = "{word} explains {word}: {string}")]
fn given_explains(class: &mut Class, name: String, activity: String, text: String) {
    explains(class, name, activity, text);
}

#[when(expr = "{word} explains {word}: {string}")]
fn when_explains(class: &mut Class, name: String, activity: String, text: String) {
    explains(class, name, activity, text);
}

fn marks(class: &mut Class) {
    let state = fold(&class.server);
    let (answer, submitted) = state.answers.iter().next_back().expect("an answer exists");
    let Some(ActivityKind::OpenResponse { rubric, .. }) =
        class.lesson.activity(&submitted.activity).map(|a| &a.kind)
    else {
        panic!("not an open response")
    };
    let assessment = assess(rubric, &submitted.text);
    let answer = *answer;
    class.run(
        "assessor",
        Command::ProposeAssessment { answer, assessment },
    );
}

#[given("the assessor marks the answer")]
fn given_marks(class: &mut Class) {
    marks(class);
}

#[when("the assessor marks the answer")]
fn when_marks(class: &mut Class) {
    marks(class);
}

#[when(expr = "{word} accepts the proposal")]
fn accepts(class: &mut Class, name: String) {
    let (proposal, _) = class.latest_proposal();
    class.run(
        &name,
        Command::DecideProposal {
            proposal,
            accept: true,
        },
    );
}

#[when(expr = "{word} undoes the assessor's mark")]
fn undoes(class: &mut Class, name: String) {
    let roster: Vec<UserId> = vec![];
    let studio = studio_view(
        &roster,
        "en",
        &class.server,
        &env(&class.lesson, &class.registry, &class.framework, class.now),
    )
    .unwrap();
    let target = studio
        .agent_actions
        .last()
        .expect("the agent did something")
        .id;
    class.run(&name, Command::Undo { target });
}

#[when(expr = "{word} switches the class to {string}")]
fn switches(class: &mut Class, name: String, framework: String) {
    class.run(
        &name,
        Command::ChangeFramework {
            framework: FrameworkId::new(framework),
        },
    );
}

#[when(expr = "{word} tries to switch the class to {string}")]
fn tries_to_switch(class: &mut Class, name: String, framework: String) {
    class.refused = class
        .try_run(
            &name,
            Command::ChangeFramework {
                framework: FrameworkId::new(framework),
            },
        )
        .err();
}

fn summative(class: &Class, learner: &str, outcome: &str, percent: u16) -> Command {
    Command::RecordEvidence {
        learner: class.user(learner),
        outcome: OutcomeId::new(outcome),
        activity: ActivityId::new("checkpoint"),
        score: Score::of(percent * 100),
        kind: EvidenceKind::Summative,
    }
}

#[when(expr = "{word} records a summative mark of {int}% for {word} on {word}")]
fn records(class: &mut Class, name: String, percent: u16, learner: String, outcome: String) {
    let command = summative(class, &learner, &outcome, percent);
    class.run(&name, command);
}

#[when(expr = "{word} tries to record a summative mark of {int}% for {word} on {word}")]
fn tries_to_record(
    class: &mut Class,
    name: String,
    percent: u16,
    learner: String,
    outcome: String,
) {
    let command = summative(class, &learner, &outcome, percent);
    class.refused = class.try_run(&name, command).err();
}

#[given(expr = "{word}'s phone is offline")]
fn offline(class: &mut Class, _name: String) {
    class.phone = Some(Phone {
        log: class.server.clone(),
        ids: SequentialIds::default(),
        outbox: vec![],
    });
}

#[when(expr = "{word}'s phone comes back online")]
fn online(class: &mut Class, _name: String) {
    let phone = class.phone.take().expect("the phone was offline");
    for (actor, envelope) in phone.outbox {
        class.sent.push((actor.clone(), envelope.clone()));
        class
            .apply(&actor, envelope)
            .expect("the server accepts what the phone accepted");
    }
    class.phone_log = Some(phone.log);
}

#[when(expr = "{word}'s phone sends its outbox again")]
fn resend(class: &mut Class, _name: String) {
    for (actor, envelope) in class.sent.clone() {
        class.apply(&actor, envelope).unwrap();
    }
}

#[then(expr = "the phone and the server agree on {word}'s progress")]
fn agree(class: &mut Class, name: String) {
    let phone = class.view_on(class.phone_log.as_ref().expect("the phone synced"), &name);
    assert_eq!(phone, class.view(&name));
}

#[then(expr = "{word}'s outcome {word} is {string}")]
fn outcome_is(class: &mut Class, name: String, outcome: String, state: String) {
    let actual = serde_json::to_value(class.outcome(&name, &outcome).judgement.state).unwrap();
    assert_eq!(actual, snake(&state));
}

#[then(expr = "{word}'s outcome {word} is open")]
fn is_open(class: &mut Class, name: String, outcome: String) {
    assert_eq!(class.outcome(&name, &outcome).lock, None);
}

#[then(expr = "{word}'s outcome {word} is locked")]
fn is_locked(class: &mut Class, name: String, outcome: String) {
    assert!(class.outcome(&name, &outcome).lock.is_some());
}

#[then(expr = "{word}'s outcome {word} is in band {string}")]
fn in_band(class: &mut Class, name: String, outcome: String, band: String) {
    assert_eq!(
        class.outcome(&name, &outcome).judgement.band.unwrap().label,
        band
    );
}

#[then(expr = "{word}'s outcome {word} has {int} piece(s) of evidence")]
fn evidence_count(class: &mut Class, name: String, outcome: String, count: usize) {
    let state = fold(&class.server);
    assert_eq!(
        state
            .evidence_for(class.user(&name), &OutcomeId::new(outcome))
            .len(),
        count
    );
}

#[then(expr = "{word} is given {word}")]
fn is_given(class: &mut Class, name: String, activity: String) {
    assert!(
        class
            .view(&name)
            .assigned
            .contains(&ActivityId::new(activity))
    );
}

#[when(expr = "{word} removes the help given to {word}")]
fn removes_help(class: &mut Class, teacher: String, learner: String) {
    let user = class.user(&learner);
    let state = fold(&class.server);
    let (id, _) = state
        .assignments
        .iter()
        .find(|(_, (l, _, _))| *l == user)
        .expect("help was given");
    let id = *id;
    class.run(&teacher, Command::Undo { target: id });
}

#[then(expr = "{word} has been given no help")]
fn no_help(class: &mut Class, name: String) {
    assert!(
        class.view(&name).assigned.is_empty(),
        "{:?}",
        class.view(&name).assigned
    );
}

#[then(expr = "{word} has been given {word} exactly once")]
fn given_once(class: &mut Class, name: String, activity: String) {
    let user = class.user(&name);
    let count = fold(&class.server)
        .assignments
        .values()
        .filter(|(l, _, a)| *l == user && a.as_str() == activity)
        .count();
    assert_eq!(count, 1);
}

fn told(class: &Class, learner: &str, outcome: &str, reason: NotifyReason) {
    let state = fold(&class.server);
    assert!(
        state
            .notifications
            .contains(&(class.user(learner), OutcomeId::new(outcome), reason)),
        "{:?}",
        state.notifications
    );
}

#[then(expr = "{word} is told {word} is struggling with {word}")]
fn told_struggling(class: &mut Class, _teacher: String, learner: String, outcome: String) {
    told(class, &learner, &outcome, NotifyReason::Struggling);
}

#[then(expr = "{word} is told {word} is below expected on {word}")]
fn told_below(class: &mut Class, _teacher: String, learner: String, outcome: String) {
    told(class, &learner, &outcome, NotifyReason::BelowExpected);
}

#[then(expr = "the proposal is {string}")]
fn proposal_is(class: &mut Class, status: String) {
    assert_eq!(
        serde_json::to_value(class.latest_proposal().1).unwrap(),
        snake(&status)
    );
}

#[then(expr = "it is refused: {word} may not {word}")]
fn refused(class: &mut Class, role: String, permission: String) {
    let Some(KernelError::Forbidden {
        role: r,
        permission: p,
    }) = &class.refused
    else {
        panic!("expected a refusal, got {:?}", class.refused)
    };
    assert_eq!(serde_json::to_value(r).unwrap(), role);
    assert_eq!(serde_json::to_value(p).unwrap(), permission);
}

fn main() {
    futures::executor::block_on(
        Class::cucumber()
            .fail_on_skipped()
            .run_and_exit(concat!(env!("CARGO_MANIFEST_DIR"), "/../../features")),
    );
}
