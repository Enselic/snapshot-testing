#!/usr/bin/env bash
set -o errexit -o nounset -o xtrace

if [ ! -f LICENSE ]; then
  echo "Run this script from the root of the snapshot-testing repository" >&2
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

SNAPSHOT_FILE=/tmp/snapshot-testing.txt
CARGO_RUN="cargo run --manifest-path test-snapshot-testing/Cargo.toml -- "
trap 'rm -f $SNAPSHOT_FILE' EXIT

assert_failure                        $CARGO_RUN banana $SNAPSHOT_FILE

assert_success env UPDATE_SNAPSHOTS=1 $CARGO_RUN banana $SNAPSHOT_FILE

assert_success                        $CARGO_RUN banana $SNAPSHOT_FILE

assert_failure                        $CARGO_RUN apple  $SNAPSHOT_FILE

assert_success env UPDATE_SNAPSHOTS=1 $CARGO_RUN apple  $SNAPSHOT_FILE

assert_success                        $CARGO_RUN apple  $SNAPSHOT_FILE
