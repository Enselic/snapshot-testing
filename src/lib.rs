/// Assert that `value` equals the snapshot at `snapshot_path`. If there is a
/// mismatch the function will panic with a helpful diff that shows what
/// changed.
///
/// Set the env var `UPDATE_SNAPSHOTS=1` to write `value` to `snapshot_file`
/// instead of asserting equality.
#[track_caller]
pub fn assert_eq_or_update(value: impl AsRef<str>, snapshot_path: impl AsRef<std::path::Path>) {
    let snapshot_path = snapshot_path.as_ref();

    if update_snapshot() {
        ensure_parent_dir_exists(snapshot_path);

        std::fs::write(snapshot_path, value.as_ref())
            .unwrap_or_else(|e| panic!("Error writing {snapshot_path:?}: {e}"));
    } else {
        let snapshot = std::fs::read_to_string(snapshot_path)
            .unwrap_or_else(|e| panic!("Error reading {snapshot_path:?}: {e}"));

        maybe_enable_colors();

        similar_asserts::assert_eq!(
            value.as_ref(),
            snapshot,
            "\n\n{}: run with env var `UPDATE_SNAPSHOTS=yes` to update snapshots\n",
            console::style("help").cyan()
        );
    }
}

/// Don't introduce new environment variables for users. Do our best based on
/// existing environment. Even better would be if `console` didn't have global
/// state, but for now we accept that that is what `similar_asserts` is using.
///
/// Also see:
/// * [`CLICOLOR_FORCE`](https://github.com/console-rs/console/blob/a51fcead7cda/src/utils.rs#L18)
/// * [`NO_COLOR`](https://github.com/console-rs/console/blob/a51fcead7cda/src/unix_term.rs#L29)
/// * [`TERM`](https://github.com/console-rs/console/blob/a51fcead7cda/src/unix_term.rs#L33)
fn maybe_enable_colors() {
    // Only care about `CARGO_TERM_COLOR=always`. Otherwise the `console`
    // defaults are fine.
    if let Ok(color) = std::env::var("CARGO_TERM_COLOR")
        && color == "always"
    {
        console::set_colors_enabled(true);
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
