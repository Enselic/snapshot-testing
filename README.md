### Usage Example

```rs
/// Regression test for `impl Display` of a complex type.
#[test]
fn test_display_of_some_type() {
    let value = ... // produce a value somehow

    snapshot_testing::assert_eq_or_update(
        value.to_string(),
        "./tests/snapshots/display-impl.txt",
    );
}
```
```sh
# Create (or update) the snapshot file
UPDATE_SNAPSHOTS=yes cargo test

# Ensure the Display impl is not accidentally changed
cargo test
```

### Diffing Engine

We use the same excellent [diffing engine](https://github.com/mitsuhiko/similar) that [`insta`](https://github.com/mitsuhiko/insta) is using. While `insta` unfortunately suffers from [Issue \#425: GitHub syntax highlights insta snapshots like Jest Snapshots](https://github.com/mitsuhiko/insta/issues/425) which makes diffs [very hard to read](https://github.com/cargo-public-api/cargo-public-api/pull/818), we allow custom snapshot file extensions and avoid that bug.

### Audit the Code

This crate is ~30 lines of code. Audit with the following one-liner, but make sure you follow to the [crates.io Data Access Policy](https://crates.io/data-access):

```sh
curl -H "User-Agent: $USER at $HOST" \
     -L https://crates.io/api/v1/crates/snapshot-testing/0.1.2/download |
         tar --extract --gzip --to-stdout | less
```

