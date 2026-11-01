#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use zidara_kernel::ids::{AgentId, UserId, Uuid};
use zidara_kernel::roles::{Actor, Permission, Role, allows};

const ROLES: [Role; 4] = [
    Role::Learner,
    Role::Teacher,
    Role::AssessmentAgent,
    Role::System,
];
const PERMISSIONS: [Permission; 8] = [
    Permission::SubmitWork,
    Permission::RecordFormative,
    Permission::RecordSummative,
    Permission::ProposeAssessment,
    Permission::DecideProposal,
    Permission::Undo,
    Permission::ChangeFramework,
    Permission::RunChallenge,
];

/// The whole table, so any change to who may do what fails here first.
#[test]
fn the_permission_table_is_exactly_this() {
    use Permission::*;
    use Role::*;
    let allowed = [
        (Learner, SubmitWork),
        (Teacher, RecordFormative),
        (Teacher, RecordSummative),
        (Teacher, DecideProposal),
        (Teacher, Undo),
        (Teacher, ChangeFramework),
        (Teacher, RunChallenge),
        (AssessmentAgent, ProposeAssessment),
        (AssessmentAgent, RecordFormative),
        (System, RecordFormative),
        (System, RecordSummative),
    ];
    for role in ROLES {
        for permission in PERMISSIONS {
            assert_eq!(
                allows(role, permission),
                allowed.contains(&(role, permission)),
                "{role:?} {permission:?}"
            );
        }
    }
}

#[test]
fn agents_never_judge_summative_work_or_decide() {
    assert!(!allows(Role::AssessmentAgent, Permission::RecordSummative));
    assert!(!allows(Role::AssessmentAgent, Permission::DecideProposal));
    assert!(!allows(Role::AssessmentAgent, Permission::Undo));
}

#[test]
fn actors_carry_their_role() {
    let teacher = Actor::Human {
        user: UserId(Uuid::from_u128(7)),
        role: Role::Teacher,
    };
    let agent = Actor::Agent {
        agent: AgentId::new("assessor"),
        role: Role::AssessmentAgent,
    };
    assert_eq!(teacher.role(), Role::Teacher);
    assert_eq!(agent.role(), Role::AssessmentAgent);
    assert_eq!(Actor::System.role(), Role::System);
    assert!(teacher.can(Permission::Undo));
    assert!(!agent.can(Permission::Undo));
}

#[test]
fn roles_and_permissions_serialize_in_snake_case() {
    assert_eq!(
        serde_json::to_string(&Role::AssessmentAgent).unwrap(),
        "\"assessment_agent\""
    );
    assert_eq!(
        serde_json::to_string(&Permission::ChangeFramework).unwrap(),
        "\"change_framework\""
    );
    let agent = Actor::Agent {
        agent: AgentId::new("assessor"),
        role: Role::AssessmentAgent,
    };
    assert_eq!(
        serde_json::to_string(&agent).unwrap(),
        r#"{"kind":"agent","agent":"assessor","role":"assessment_agent"}"#
    );
}
