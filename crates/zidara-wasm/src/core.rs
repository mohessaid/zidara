//! The phone's kernel: the same `decide` as the server, over a local log. Plain Rust with a
//! JSON contract, so it is tested natively; `Shell` only hands it to JavaScript.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zidara_frameworks::{Registry, demo_registry};
use zidara_kernel::activity::{ActivityKind, Response, version};
use zidara_kernel::decide::{Env, Envelope, KernelError, decide, next_variant};
use zidara_kernel::event::Event;
use zidara_kernel::framework::FrameworkRegistry;
use zidara_kernel::governance::{Assessment, Policy, rubric_marks};
use zidara_kernel::ids::{ActivityId, CohortId, EventId, FrameworkId, IdGen, TenantId, UserId};
use zidara_kernel::lesson::Lesson;
use zidara_kernel::roles::{Actor, Role};
use zidara_kernel::view::{learner_view, studio_view};
use zidara_kernel::visibility::visible_to;

#[derive(Deserialize)]
struct Setup {
    lesson: Lesson,
    framework: FrameworkId,
    tenant: Uuid,
    cohort: Uuid,
}

#[derive(Serialize)]
struct Mark {
    answer: EventId,
    assessment: Assessment,
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum RunResult<'a> {
    Applied { events: &'a [Event] },
    Duplicate,
    Refused { error: KernelError },
}

/// Event ids derived from the envelope id: no randomness in wasm, and the same session always
/// gives the same ids. They are provisional; the server assigns its own on sync.
struct Derived {
    base: u128,
    n: u128,
}

impl IdGen for Derived {
    fn next_id(&mut self) -> EventId {
        self.n += 1;
        EventId(Uuid::from_u128(self.base.wrapping_add(self.n)))
    }
}

#[derive(Debug)]
pub struct Core {
    lesson: Lesson,
    registry: Registry,
    framework: FrameworkId,
    tenant: TenantId,
    cohort: CohortId,
    log: Vec<Event>,
    applied: BTreeSet<Uuid>,
}

fn parse<'a, T: Deserialize<'a>>(what: &str, json: &'a str) -> Result<T, String> {
    serde_json::from_str(json).map_err(|e| format!("{what}: {e}"))
}

fn to_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("kernel types always serialize")
}

impl Core {
    pub fn new(setup: &str) -> Result<Core, String> {
        let setup: Setup = parse("setup", setup)?;
        let registry = demo_registry();
        if registry.get(&setup.framework).is_none() {
            return Err(format!(
                "setup: unknown framework {:?}",
                setup.framework.as_str()
            ));
        }
        Ok(Core {
            lesson: setup.lesson,
            registry,
            framework: setup.framework,
            tenant: TenantId(setup.tenant),
            cohort: CohortId(setup.cohort),
            log: Vec::new(),
            applied: BTreeSet::new(),
        })
    }

    /// Replaces the local log, e.g. with the server's after a sync.
    pub fn load(&mut self, events: &str) -> Result<(), String> {
        self.log = parse("events", events)?;
        self.applied.clear();
        Ok(())
    }

    pub fn events(&self) -> String {
        to_json(&self.log)
    }

    /// Runs one command as `actor`, at the envelope's time.
    pub fn run(&mut self, actor: &str, envelope: &str) -> Result<String, String> {
        let actor: Actor = parse("actor", actor)?;
        let envelope: Envelope = parse("envelope", envelope)?;
        if self.applied.contains(&envelope.id) {
            return Ok(to_json(&RunResult::Duplicate));
        }
        let env = self.env(envelope.at);
        let mut ids = Derived {
            base: envelope.id.as_u128(),
            n: 0,
        };
        match decide(&actor, envelope.command, &self.log, &env, &mut ids) {
            Ok(events) => {
                self.applied.insert(envelope.id);
                let result = to_json(&RunResult::Applied { events: &events });
                self.log.extend(events);
                Ok(result)
            }
            Err(error) => Ok(to_json(&RunResult::Refused { error })),
        }
    }

    /// The log as `user` in `role` may read it (what the server's database would return).
    pub fn visible_events(&self, user: &str, role: &str) -> Result<String, String> {
        let user = Uuid::parse_str(user).map_err(|e| format!("user: {e}"))?;
        let role: Role = serde_json::from_value(serde_json::Value::String(role.into()))
            .map_err(|e| format!("role: {e}"))?;
        Ok(to_json(&visible_to(&self.log, UserId(user), role)))
    }

    /// The rubric's mark for each open answer with no proposal yet: `[{ answer, assessment }]`.
    pub fn rubric_marks(&self) -> String {
        let marks: Vec<Mark> = rubric_marks(&self.log, &self.lesson)
            .into_iter()
            .map(|(answer, assessment)| Mark { answer, assessment })
            .collect();
        to_json(&marks)
    }

    /// The version of a quiz the learner must be shown next; the kernel refuses any other.
    pub fn next_variant(&self, user: &str, activity: &str) -> Result<u32, String> {
        let user = Uuid::parse_str(user).map_err(|e| format!("user: {e}"))?;
        Ok(next_variant(
            &self.log,
            UserId(user),
            &ActivityId::new(activity),
            &self.lesson,
        ))
    }

    /// Whether `response` (JSON) answers question `index` of version `variant` of a quiz: the
    /// kernel's own marking, for feedback the moment a learner answers. It records nothing;
    /// evidence still comes only from submitting the quiz.
    pub fn check(
        &self,
        activity: &str,
        variant: u32,
        index: u32,
        response: &str,
    ) -> Result<bool, String> {
        let response: Response = parse("response", response)?;
        let ActivityKind::Quiz {
            questions,
            variants,
            ..
        } = &self
            .lesson
            .activity(&ActivityId::new(activity))
            .ok_or_else(|| format!("no quiz {activity}"))?
            .kind
        else {
            return Err(format!("no quiz {activity}"));
        };
        version(questions, variants, variant)
            .and_then(|asked| asked.get(index as usize))
            .map(|question| question.expected.accepts(&response))
            .ok_or_else(|| format!("no question {index} in version {variant} of {activity}"))
    }

    /// Views are drawn at `now`, the shell's clock (calendar locks depend on it).
    pub fn learner_view(&self, user: &str, locale: &str, now: i64) -> Result<String, String> {
        let user = Uuid::parse_str(user).map_err(|e| format!("user: {e}"))?;
        let env = self.env(zidara_kernel::ids::Timestamp(now));
        let view = learner_view(UserId(user), locale, &self.log, &env)
            .map_err(|e| format!("view: {e}"))?;
        Ok(to_json(&view))
    }

    pub fn studio_view(&self, roster: &str, locale: &str, now: i64) -> Result<String, String> {
        let roster: Vec<UserId> = parse("roster", roster)?;
        let env = self.env(zidara_kernel::ids::Timestamp(now));
        let view =
            studio_view(&roster, locale, &self.log, &env).map_err(|e| format!("view: {e}"))?;
        Ok(to_json(&view))
    }

    fn env(&self, now: zidara_kernel::ids::Timestamp) -> Env<'_> {
        Env {
            now,
            tenant: self.tenant,
            cohort: self.cohort,
            lesson: &self.lesson,
            frameworks: &self.registry,
            default_framework: &self.framework,
            policy: Policy::STANDARD,
        }
    }
}
