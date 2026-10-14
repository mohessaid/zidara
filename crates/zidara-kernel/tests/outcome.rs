mod support;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test as test;

use proptest::prelude::*;
use zidara_kernel::ids::OutcomeId;
use zidara_kernel::locale::Localized;
use zidara_kernel::outcome::{GraphError, Outcome, OutcomeGraph};

fn id(s: &str) -> OutcomeId {
    OutcomeId::new(s)
}

fn o(name: &str, needs: &[&str]) -> Outcome {
    Outcome {
        id: id(name),
        title: Localized::from_pairs(&[("en", name)]),
        prerequisites: needs.iter().map(|n| id(n)).collect(),
    }
}

fn order(graph: &OutcomeGraph) -> Vec<&str> {
    graph.iter().map(|o| o.id.as_str()).collect()
}

#[test]
fn puts_prerequisites_first() {
    let graph = OutcomeGraph::new(vec![o("F3", &["F2"]), o("F1", &[]), o("F2", &["F1"])]).unwrap();
    assert_eq!(order(&graph), ["F1", "F2", "F3"]);
    assert_eq!(graph.len(), 3);
    assert!(!graph.is_empty());
}

#[test]
fn keeps_input_order_among_independent_outcomes() {
    let graph = OutcomeGraph::new(vec![o("B", &[]), o("A", &[])]).unwrap();
    assert_eq!(order(&graph), ["B", "A"]);
}

#[test]
fn an_empty_graph_is_allowed() {
    let graph = OutcomeGraph::new(vec![]).unwrap();
    assert!(graph.is_empty());
    assert_eq!(graph.len(), 0);
}

#[test]
fn finds_outcomes_by_id() {
    let graph = OutcomeGraph::new(vec![o("F1", &[]), o("F2", &["F1"])]).unwrap();
    assert_eq!(graph.get(&id("F2")).unwrap().prerequisites, vec![id("F1")]);
    assert!(graph.get(&id("F9")).is_none());
}

#[test]
fn rejects_duplicates() {
    let err = OutcomeGraph::new(vec![o("F1", &[]), o("F1", &[])]).unwrap_err();
    assert_eq!(err, GraphError::Duplicate(id("F1")));
}

#[test]
fn rejects_unknown_prerequisites() {
    let err = OutcomeGraph::new(vec![o("F1", &[]), o("F2", &["F1", "F9"])]).unwrap_err();
    assert_eq!(
        err,
        GraphError::UnknownPrerequisite {
            outcome: id("F2"),
            missing: id("F9")
        }
    );
}

#[test]
fn rejects_cycles() {
    let err = OutcomeGraph::new(vec![o("A", &["B"]), o("B", &["A"])]).unwrap_err();
    assert_eq!(err, GraphError::Cycle(id("A")));
    let err = OutcomeGraph::new(vec![o("C", &[]), o("A", &["A"])]).unwrap_err();
    assert_eq!(err, GraphError::Cycle(id("A")));
}

#[test]
fn reads_case_shaped_json() {
    let json = r#"[{"id":"F1","title":{"en":"Parts"}},{"id":"F2","title":{"en":"Equal"},"prerequisites":["F1"]}]"#;
    let outcomes: Vec<Outcome> = serde_json::from_str(json).unwrap();
    let graph = OutcomeGraph::new(outcomes).unwrap();
    assert_eq!(order(&graph), ["F1", "F2"]);
}

/// Any acyclic graph, given in any order, comes out with every prerequisite first.
#[test]
fn every_prerequisite_comes_first() {
    let dag = (1usize..10)
        .prop_flat_map(|n| {
            proptest::collection::vec(proptest::collection::vec(any::<bool>(), n), n)
        })
        .prop_map(|edges| {
            edges
                .iter()
                .enumerate()
                .map(|(i, row)| {
                    let name = format!("O{i}");
                    let needs: Vec<String> = (0..i)
                        .filter(|j| row[*j])
                        .map(|j| format!("O{j}"))
                        .collect();
                    let needs: Vec<&str> = needs.iter().map(String::as_str).collect();
                    o(&name, &needs)
                })
                .collect::<Vec<_>>()
        })
        .prop_shuffle();
    support::check(dag, |outcomes| {
        let graph = OutcomeGraph::new(outcomes.clone()).unwrap();
        prop_assert_eq!(graph.len(), outcomes.len());
        let position = |x: &OutcomeId| graph.iter().position(|o| &o.id == x).unwrap();
        for outcome in graph.iter() {
            for needed in &outcome.prerequisites {
                prop_assert!(position(needed) < position(&outcome.id));
            }
        }
        Ok(())
    });
}

#[test]
fn errors_explain_themselves() {
    assert_eq!(
        GraphError::Cycle(id("A")).to_string(),
        "outcome \"A\" is part of a prerequisite cycle"
    );
}
