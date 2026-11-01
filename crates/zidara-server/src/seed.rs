//! The demo school: one tenant, two classes that must never see each other's data, their
//! learners and teachers, two open classes for guests, and the assessment agent that serves all.

use sqlx::PgPool;
use uuid::Uuid;

/// Fixed ids, so demos and tests can name people.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Seed {
    pub tenant: Uuid,
    pub a: Uuid,
    pub b: Uuid,
    pub amina: Uuid,
    pub yacine: Uuid,
    pub sara: Uuid,
    pub karim: Uuid,
    pub nadia: Uuid,
    pub agent: Uuid,
    /// The open class: anyone can learn how Zidara works, as Lina, with Omar teaching.
    pub c: Uuid,
    pub lina: Uuid,
    pub omar: Uuid,
    /// The open fractions class: guests learn equivalent fractions here (ADR 0012).
    pub d: Uuid,
}

impl Seed {
    pub const DEMO: Seed = Seed {
        tenant: Uuid::from_u128(0x7e4a_0001),
        a: Uuid::from_u128(0xc0_0a),
        b: Uuid::from_u128(0xc0_0b),
        amina: Uuid::from_u128(0x1e_a001),
        yacine: Uuid::from_u128(0x1e_a002),
        sara: Uuid::from_u128(0x1e_b001),
        karim: Uuid::from_u128(0x7e_a001),
        nadia: Uuid::from_u128(0x7e_b001),
        agent: Uuid::from_u128(0xa6_0001),
        c: Uuid::from_u128(0xc0_0c),
        lina: Uuid::from_u128(0x1e_c001),
        omar: Uuid::from_u128(0x7e_c001),
        d: Uuid::from_u128(0xc0_0d),
    };
}

/// Inserts the demo school. Run as the owner.
pub async fn seed(owner: &PgPool) -> Result<Seed, sqlx::Error> {
    let s = Seed::DEMO;
    let mut tx = owner.begin().await?;
    sqlx::query("insert into tenants (id, name) values ($1, 'Demo school')")
        .bind(s.tenant)
        .execute(&mut *tx)
        .await?;
    // Open classes take guests (ADR 0012).
    for (id, name, lesson, open) in [
        (s.a, "Class 4 A", "equivalent-fractions", false),
        (s.b, "Class 4 B", "equivalent-fractions", false),
        (s.c, "Discover Zidara", "how-zidara-works", true),
        (s.d, "Open fractions", "equivalent-fractions", true),
    ] {
        sqlx::query("insert into cohorts (id, tenant_id, name, lesson_id, framework, open) values ($1, $2, $3, $4, 'mastery', $5)")
            .bind(id)
            .bind(s.tenant)
            .bind(name)
            .bind(lesson)
            .bind(open)
            .execute(&mut *tx)
            .await?;
    }
    let people = [
        (s.amina, "Amina", "person"),
        (s.yacine, "Yacine", "person"),
        (s.sara, "Sara", "person"),
        (s.karim, "Karim", "person"),
        (s.nadia, "Nadia", "person"),
        (s.lina, "Lina", "person"),
        (s.omar, "Omar", "person"),
        (s.agent, "assessor", "agent"),
    ];
    for (id, name, kind) in people {
        sqlx::query(
            "insert into users (id, tenant_id, display_name, kind) values ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(s.tenant)
        .bind(name)
        .bind(kind)
        .execute(&mut *tx)
        .await?;
    }
    let memberships = [
        (s.amina, s.a, "learner"),
        (s.yacine, s.a, "learner"),
        (s.sara, s.b, "learner"),
        (s.karim, s.a, "teacher"),
        (s.nadia, s.b, "teacher"),
        (s.lina, s.c, "learner"),
        (s.omar, s.c, "teacher"),
        (s.agent, s.a, "assessment_agent"),
        (s.agent, s.b, "assessment_agent"),
        (s.agent, s.c, "assessment_agent"),
        (s.agent, s.d, "assessment_agent"),
    ];
    for (user, cohort, role) in memberships {
        sqlx::query("insert into memberships (user_id, cohort_id, role) values ($1, $2, $3)")
            .bind(user)
            .bind(cohort)
            .bind(role)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(s)
}
