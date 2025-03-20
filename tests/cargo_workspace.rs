use cocogitto_depgraph::DepGraphResolver;
use speculoos::prelude::*;

// ┌───────────┐     ┌───────────┐
// │ package-b │ ◀── │ package-a │
// └───────────┘     └───────────┘
// │
// │
// ▼
// ┌───────────┐
// │ package-c │ ─┐
// └───────────┘  │
// │              │
// │              │
// ▼              │
// ┌───────────┐  │
// │ package-d │  │
// └───────────┘  │
// │              │
// │              │
// ▼              │
// ┌───────────┐  │
// │ package-e │ ◀┘
// └───────────┘

#[test]
fn cargo_workspace() {
    let resolver = DepGraphResolver::Cargo;
    let dependencies = resolver.topological_sort("tests/lang/cargo_workspace/Cargo.toml");
    let dependencies: Vec<_> = dependencies
        .iter()
        .map(String::as_str)
        .collect();

    assert_that!(dependencies).is_equal_to(vec![
        "package-e",
        "package-d",
        "package-c",
        "package-b",
        "package-a",
    ])
}
