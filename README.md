### Usage

Write a test:

```rs
#[test]
fn check_snapshot() {
    let value = "... contents to regression test ...";
    snapshot_testing::assert_eq_or_update(value, "snapshot.txt");
}
```

Update the snapshot file before the first test:

```sh
UPDATE_SNAPSHOTS=yes cargo test
```

Assert that current contents matches the contents of the snapshot file:

```sh
cargo test
```

### Diffing Engine

We use the same [excellent diffing engine](https://github.com/mitsuhiko/similar) that [insta](https://github.com/mitsuhiko/insta) is using. But since we allow custom snapshot file extensions we do not suffer from [Consider ways to avoid conflicts with Jest snapshots for external tools (like GitHub Linguist)](https://github.com/mitsuhiko/insta/issues/425). That bug makes GitHub diffs [horrible](https://github.com/trishume/syntect/pull/591/files#diff-f3d28a949326c02c2b2c85c024a030561f3b2521457621bb2550792460c3a2ec). Unfortunately the `.gitattributes` workaround [does not work](https://github.com/trishume/syntect/pull/592#issuecomment-3153214380).

### Audit the Code

This crate is ~30 lines of code. Audit with the following one-liner, but make sure you follow to the [crates.io Data Access Policy](https://crates.io/data-access):

```sh
curl -H "User-Agent: $USER at $HOST" \
     -L https://crates.io/api/v1/crates/snapshot-testing/0.1.0/download |
         tar --extract --gzip --to-stdout | less
```

