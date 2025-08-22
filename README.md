## snapshot-testing

Assert that strings equal easily updatable snapshot files. Show nice colored diffs if not.

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

We use the excellent [`insta`](https://github.com/mitsuhiko/insta) [diffing engine](https://github.com/mitsuhiko/similar) without suffering from [Issue \#425: GitHub syntax highlights insta snapshots like Jest Snapshots](https://github.com/mitsuhiko/insta/issues/425) which unfortunately makes diffs [very hard to read](https://github.com/cargo-public-api/cargo-public-api/pull/818).

### Audit the Code

This crate is very small and easily audited with the following command [^1]:

```sh
curl -H "User-Agent: $USER at $HOST" \
     -L https://crates.io/api/v1/crates/snapshot-testing/0.1.8/download |
tar --extract --gzip --to-stdout |
less
```

[^1]: Please also see [crates.io Data Access Policy](https://crates.io/data-access).
