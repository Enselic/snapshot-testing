#!/usr/bin/env bash
set -o errexit -o nounset -o xtrace

if [ ! -f LICENSE ]; then
  echo "Run this script from the root of the repository" >&2
  exit 1
fi

# First do some linting
cargo fmt -- --check
RUSTDOCFLAGS='--deny warnings' cargo doc --locked --no-deps --document-private-items
cargo clippy

# Then run the tests
assert_failure() {
  if "$@"; then
    echo "Expected failure, but command succeeded: $*" >&2
    exit 1
  fi
}

assert_success() {
  if ! "$@"; then
    echo "Expected success, but command failed: $*" >&2
    exit 1
  fi
}

SNAPSHOT_FILE=./tests/snapshots/display-impl.txt
CARGO_TEST="cargo test"

# Assert failure when there is no snapshot file
rm -f $SNAPSHOT_FILE
trap "rm -f $SNAPSHOT_FILE" EXIT
assert_failure env SNAPSHOT_VALUE=banana                    $CARGO_TEST

# Assert that it is possible to create a snapshot
assert_success env SNAPSHOT_VALUE=banana UPDATE_SNAPSHOTS=1 $CARGO_TEST

# Assert that with a snapshot the test passes
assert_success env SNAPSHOT_VALUE=banana                    $CARGO_TEST

# Assert that we fail if the snapshot changes
assert_failure env SNAPSHOT_VALUE=apple                     $CARGO_TEST

# Assert that we can update the snapshot
assert_success env SNAPSHOT_VALUE=apple  UPDATE_SNAPSHOTS=1 $CARGO_TEST

# Assert that we pass with an updated snapshot
assert_success env SNAPSHOT_VALUE=apple                     $CARGO_TEST
