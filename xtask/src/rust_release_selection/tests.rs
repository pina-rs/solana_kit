//! Focused selection checks for the Rust release selection guard.

use super::{
    Status, evaluate,
    inputs::{
        Decision, Dependency, Metadata, Package, Plan, PlanBatch, PlanPackage, Publication,
        Readiness, ReleaseRecord, Selected,
    },
};
use std::collections::BTreeSet;

/// Builds one reviewed plan entry.
fn plan_package(package: &str, decision: Decision) -> PlanPackage {
    PlanPackage {
        package: package.into(),
        version: "0.1.0".into(),
        decision,
        reason: "test fixture".into(),
    }
}

/// Builds one workspace manifest package.
fn manifest(package: &str, publish: Option<Vec<String>>) -> Package {
    Package {
        name: package.into(),
        version: "0.1.0".into(),
        publish,
        dependencies: Vec::new(),
    }
}

/// Builds one crates.io publication entry.
fn publication(package: &str) -> Publication {
    Publication {
        package: package.into(),
        ecosystem: "cargo".into(),
        registry: "crates_io".into(),
        version: "0.1.0".into(),
        mode: "builtin".into(),
    }
}

/// Builds one readiness selection row.
fn selected(package: &str) -> Selected {
    Selected {
        package: package.into(),
        ecosystem: Some("cargo".into()),
        registry: Some("crates_io".into()),
        version: "0.1.0".into(),
        status: "ready".into(),
    }
}

/// The exact two-package control: one public crate, one private fixture.
struct Fixture {
    plan: Plan,
    configured: BTreeSet<String>,
    metadata: Metadata,
    release: ReleaseRecord,
    readiness: Readiness,
}

impl Fixture {
    fn new() -> Self {
        Self {
            plan: Plan {
                schema_version: 1,
                kind: "solanaKit.rustReleasePlan".into(),
                scope: Some("unit control fixture".into()),
                packages: vec![
                    plan_package("core", Decision::Public),
                    plan_package("private", Decision::PrivateFixture),
                ],
                batches: vec![PlanBatch {
                    index: 1,
                    packages: vec!["core".into()],
                }],
            },
            configured: BTreeSet::from(["core".into(), "private".into()]),
            metadata: Metadata {
                packages: vec![manifest("core", None), manifest("private", Some(vec![]))],
            },
            release: ReleaseRecord {
                kind: "monochange.releaseRecord".into(),
                package_publications: vec![publication("core")],
            },
            readiness: Readiness {
                kind: "monochange.publishReadiness".into(),
                record_commit: None,
                packages: vec![selected("core")],
            },
        }
    }

    fn report(&self) -> super::Report {
        evaluate(
            &self.plan,
            &self.configured,
            &self.metadata,
            &self.release,
            &self.readiness,
            None,
        )
    }

    /// Requires one blocker containing `needle`, printing all blockers on failure.
    fn assert_blocker(&self, needle: &str) {
        let report = self.report();
        assert!(
            report
                .blockers
                .iter()
                .any(|blocker| blocker.contains(needle)),
            "no blocker contains {needle:?}; all blockers: {:?}",
            report.blockers
        );
    }
}

#[test]
fn exact_selection_passes_as_unit_control() {
    let report = Fixture::new().report();

    assert_eq!(report.status, Status::SelectionExact);
    assert_eq!(report.selection.status, "exact");
    assert_eq!(report.release_record.status, "exact");
    assert_eq!(report.selection.expected.len(), 1);
    assert!(report.blockers.is_empty(), "{:?}", report.blockers);
    assert_eq!(report.release_record.expected_publications, 1);
    assert_eq!(report.batches.len(), 1);
}

#[test]
fn empty_selection_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages.clear();

    assert_eq!(fixture.report().selection.status, "empty");
    fixture.assert_blocker("zero selected packages");
}

#[test]
fn wrong_version_selection_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages[0].version = "0.2.0".into();

    fixture.assert_blocker("readiness selection has wrong version core 0.2.0");
}

#[test]
fn duplicate_selection_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages.push(selected("core"));

    assert_eq!(fixture.report().selection.duplicates, ["core"]);
    fixture.assert_blocker("readiness selection contains duplicate package core");
}

#[test]
fn invented_selection_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages.push(selected("invented"));

    fixture.assert_blocker("readiness selection contains unexpected package invented");
}

#[test]
fn private_package_in_selection_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages.push(selected("private"));

    fixture.assert_blocker("readiness selection contains unexpected package private");
}

#[test]
fn missing_expected_package_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages.clear();

    fixture.assert_blocker("readiness selection omits expected package core");
}

#[test]
fn wrong_registry_target_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages[0].registry = Some("npm".into());

    assert_eq!(fixture.report().selection.wrong_targets, ["core"]);
    fixture.assert_blocker("readiness selection has wrong registry target for core");
}

#[test]
fn non_ready_status_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages[0].status = "held".into();

    fixture.assert_blocker("readiness selection is not ready for core: held");
}

#[test]
fn empty_publications_block() {
    let mut fixture = Fixture::new();
    fixture.release.package_publications.clear();
    let report = fixture.report();

    assert_eq!(report.release_record.status, "empty");
    fixture.assert_blocker("release record contains no package publications");
}

#[test]
fn wrong_version_publication_blocks() {
    let mut fixture = Fixture::new();
    fixture.release.package_publications[0].version = "0.2.0".into();

    fixture.assert_blocker("release publication version mismatch: core 0.2.0 (expected 0.1.0)");
}

#[test]
fn unexpected_publication_blocks() {
    let mut fixture = Fixture::new();
    fixture
        .release
        .package_publications
        .push(publication("private"));

    fixture.assert_blocker("release record publishes non-public package private");
}

#[test]
fn wrong_registry_publication_blocks() {
    let mut fixture = Fixture::new();
    fixture.release.package_publications[0].registry = "npm".into();

    fixture.assert_blocker("release publication has wrong target core registry npm");
}

#[test]
fn duplicate_publication_blocks() {
    let mut fixture = Fixture::new();
    fixture
        .release
        .package_publications
        .push(publication("core"));

    assert_eq!(fixture.report().release_record.duplicates, ["core"]);
    fixture.assert_blocker("release record contains duplicate publication for core");
}

#[test]
fn duplicate_plan_entry_blocks() {
    let mut fixture = Fixture::new();
    fixture
        .plan
        .packages
        .push(plan_package("core", Decision::Public));

    fixture.assert_blocker("reviewed plan contains duplicate package core");
}

#[test]
fn invented_plan_entry_blocks() {
    let mut fixture = Fixture::new();
    fixture
        .plan
        .packages
        .push(plan_package("invented", Decision::PrivateDeferred));

    fixture.assert_blocker("reviewed plan contains invented package invented");
}

#[test]
fn omitted_configured_package_blocks() {
    let mut fixture = Fixture::new();
    fixture.configured.insert("omitted".into());

    fixture.assert_blocker("configured Cargo package omitted is omitted from the reviewed plan");
}

#[test]
fn version_drift_blocks() {
    let mut fixture = Fixture::new();
    fixture.metadata.packages[0].version = "0.2.0".into();

    fixture.assert_blocker("reviewed version for core is 0.1.0, workspace manifest has 0.2.0");
}

#[test]
fn public_package_with_disabled_publish_blocks() {
    let mut fixture = Fixture::new();
    fixture.metadata.packages[0].publish = Some(vec![]);

    fixture.assert_blocker("public package core has Cargo publication disabled");
}

#[test]
fn private_package_with_allowed_publish_blocks() {
    let mut fixture = Fixture::new();
    fixture.metadata.packages[1].publish = None;

    fixture.assert_blocker("private package private has Cargo publication allowed");
}

#[test]
fn blocked_package_requires_git_dependency() {
    let mut fixture = Fixture::new();
    fixture.plan.packages[1] = PlanPackage {
        package: "leptoskit".into(),
        version: "0.1.0".into(),
        decision: Decision::BlockedUpstream,
        reason: "test fixture".into(),
    };
    fixture.configured.insert("leptoskit".into());
    fixture.metadata.packages.push(manifest("leptoskit", None));

    fixture.assert_blocker("blocked package leptoskit has no unpublished Git dependency");
}

#[test]
fn git_dependency_blocks_public_package() {
    let mut fixture = Fixture::new();
    fixture.metadata.packages[0].dependencies = vec![git_dependency("bridge")];

    fixture.assert_blocker("package core has unpublished Git dependency bridge");
}

#[test]
fn dev_git_dependency_is_ignored() {
    let mut fixture = Fixture::new();
    let mut dependency = git_dependency("bridge");
    dependency.kind = Some("dev".into());
    fixture.metadata.packages[0].dependencies = vec![dependency];

    assert!(fixture.report().blockers.is_empty());
}

#[test]
fn public_dependency_on_private_package_blocks() {
    let mut fixture = Fixture::new();
    fixture.metadata.packages[0].dependencies = vec![Dependency {
        name: "private".into(),
        source: None,
        kind: None,
    }];

    fixture.assert_blocker("public package core depends on non-public package private");
}

#[test]
fn batch_partition_mismatch_blocks() {
    let mut fixture = Fixture::new();
    fixture.plan.batches.clear();

    fixture.assert_blocker("reviewed batches omit public package core");
}

#[test]
fn batch_with_private_package_blocks() {
    let mut fixture = Fixture::new();
    fixture.plan.batches[0].packages.push("private".into());

    fixture.assert_blocker("reviewed batches include non-public package private");
}

#[test]
fn wrong_release_record_kind_blocks() {
    let mut fixture = Fixture::new();
    fixture.release.kind = "something.else".into();

    fixture.assert_blocker("release record kind is something.else");
}

#[test]
fn wrong_plan_kind_blocks() {
    let mut fixture = Fixture::new();
    fixture.plan.kind = "something.else".into();

    fixture.assert_blocker("reviewed plan kind is something.else");
}

#[test]
fn plan_entry_without_reason_blocks() {
    let mut fixture = Fixture::new();
    fixture.plan.packages[0].reason = "  ".into();

    fixture.assert_blocker("reviewed plan entry core has no recorded reason");
}

#[test]
fn wrong_readiness_kind_blocks() {
    let mut fixture = Fixture::new();
    fixture.readiness.kind = "something.else".into();

    fixture.assert_blocker("readiness kind is something.else");
}

#[test]
fn record_commit_binding_requires_exact_match() {
    let mut fixture = Fixture::new();
    fixture.readiness.record_commit = Some("commit-a".into());

    let matching = evaluate(
        &fixture.plan,
        &fixture.configured,
        &fixture.metadata,
        &fixture.release,
        &fixture.readiness,
        Some("commit-a"),
    );
    assert!(matching.blockers.is_empty(), "{:?}", matching.blockers);

    let mismatching = evaluate(
        &fixture.plan,
        &fixture.configured,
        &fixture.metadata,
        &fixture.release,
        &fixture.readiness,
        Some("commit-b"),
    );
    assert!(
        mismatching
            .blockers
            .iter()
            .any(|blocker| blocker.contains("does not match the configured release record commit")),
    );
}

#[test]
fn plan_without_public_packages_blocks() {
    let mut fixture = Fixture::new();
    fixture.plan.packages[0].decision = Decision::PrivateDeferred;

    fixture.assert_blocker("reviewed plan has no public packages");
}

/// Builds one Git checkout dependency row.
fn git_dependency(name: &str) -> Dependency {
    Dependency {
        name: name.into(),
        source: Some("git+https://example.invalid/bridge".into()),
        kind: None,
    }
}

#[test]
fn real_monochange_metadata_fields_do_not_change_selection() {
    let mut fixture = Fixture::new();
    fixture.readiness.packages[0] = serde_json::from_value(serde_json::json!({
        "package": "core", "ecosystem": "cargo", "registry": "crates_io",
        "version": "0.1.0", "status": "ready", "message": "would publish core",
        "trusted_publishing": {"status": "disabled", "message": "uses registry tokens"}
    }))
    .expect("actual Monochange readiness row shape");
    fixture.release.package_publications[0] = serde_json::from_value(serde_json::json!({
        "package": "core", "ecosystem": "cargo", "registry": "crates_io",
        "version": "0.1.0", "mode": "builtin",
        "trusted_publishing": {"enabled": false, "mode": "required"}
    }))
    .expect("actual Monochange publication row shape");

    assert_eq!(fixture.report().status, Status::SelectionExact);
}

#[test]
fn report_cannot_overwrite_an_input() {
    use std::fs;

    let directory =
        std::env::temp_dir().join(format!("rust-selection-output-{}", std::process::id()));
    fs::create_dir_all(&directory).expect("private test directory");
    let input = directory.join("input.json");
    fs::write(&input, b"original input").expect("input fixture");
    let config = super::Config {
        plan: input.clone(),
        monochange: input.clone(),
        release_record: input.clone(),
        readiness: input.clone(),
        record_commit: None,
        output: Some(input.clone()),
    };
    let error = super::validate_paths(&config, &input).expect_err("input overwrite rejected");

    assert!(error.to_string().contains("overwrite an input"));
    assert_eq!(
        fs::read(&input).expect("preserved input"),
        b"original input"
    );
    fs::remove_dir_all(directory).expect("remove test directory");
}
