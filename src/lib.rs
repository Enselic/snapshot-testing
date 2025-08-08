/// Assert that `value` matches the snapshot at `snapshot_path`. If there is a
/// mismatch the function will panic with a helpful diff that shows what
/// changed.
///
/// If the env var `UPDATE_SNAPSHOTS` is set to `1`, `yes` or `true` then
/// `value` will be written to `snapshot_file` instead of being asserted to
/// match.
///
/// Set the env var `CLICOLOR_FORCE` to `1` to force colors in diffs in e.g. CI
/// logs. See <https://github.com/console-rs/console/blob/a51fcead7cda/src/utils.rs#L18>
/// which is what our dependency `similar-asserts` uses.
#[track_caller]
pub fn assert_eq_or_update(value: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
    let value = value.as_ref();
    let snapshot_path = snapshot_path.as_ref();

    if update_snapshot() {
        ensure_parent_dir_exists(snapshot_path);

        std::fs::write(snapshot_path, value)
            .unwrap_or_else(|e| panic!("Error writing {snapshot_path:?}: {e}"));
    } else {
        let snapshot = std::fs::read_to_string(snapshot_path)
            .unwrap_or_else(|e| panic!("Error reading {snapshot_path:?}: {e}"));

        similar_asserts::assert_eq!(
            value,
            snapshot,
            "\n\n{}: Run with env var `UPDATE_SNAPSHOTS=yes` to update snapshots\n",
            console::style("help").cyan()
        );
    }
}

fn ensure_parent_dir_exists(snapshot_path: &std::path::Path) {
    if !snapshot_path.exists() {
        std::fs::create_dir_all(snapshot_path.parent().unwrap())
            .unwrap_or_else(|e| panic!("Error creating directory for {snapshot_path:?}: {e}"));
    }
}

fn update_snapshot() -> bool {
    std::env::var("UPDATE_SNAPSHOTS")
        .map(|s| s.to_lowercase())
        .is_ok_and(|s| s == "1" || s == "yes" || s == "true")
}
