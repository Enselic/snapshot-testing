/// Assert that `value` matches the snapshot at `snapshot_path`. If there is a
/// diff the function will panic with a colorful diff that shows what changed.
///
/// If the env var `UPDATE_SNAPSHOTS` is set to `1`, `yes` or `true` then
/// `value` will be written to `snapshot_file` instead of being asserted to
/// match.
pub fn assert_eq_or_update(value: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
    let value = value.as_ref();
    let snapshot_path = snapshot_path.as_ref();

    if update_snapshots() {
        std::fs::write(snapshot_path, value)
            .unwrap_or_else(|e| panic!("Error writing `{snapshot_path:?}`: {e}"));
    } else {
        let snapshot = std::fs::read_to_string(snapshot_path)
            .unwrap_or_else(|e| panic!("Error reading `{snapshot_path:?}`: {e}"));

        similar_asserts::assert_eq!(value, snapshot);
    }
}

fn update_snapshots() -> bool {
    std::env::var("UPDATE_SNAPSHOTS")
        .map(|s| s.to_lowercase())
        .is_ok_and(|s| s == "1" || s == "yes" || s == "true")
}
