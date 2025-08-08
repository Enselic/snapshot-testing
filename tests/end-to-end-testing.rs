struct SnapshotValue;

impl std::fmt::Display for SnapshotValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::env::var("SNAPSHOT_VALUE").unwrap_or_default())
    }
}

#[test]
fn test_display_of_some_type() {
    let value = SnapshotValue;

    snapshot_testing::assert_eq_or_update(value.to_string(), "./tests/snapshots/display-impl.txt");
}

#[test]
fn testing_through_test_sh_script() {
    assert!(
        std::env::var("SNAPSHOT_VALUE").is_ok(),
        "ERROR: You must run tests through ./tests/test.sh"
    );
}
