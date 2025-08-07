#!/usr/bin/env bash
set -o errexit -o nounset -o xtrace

cargo fmt -- --check

RUSTDOCFLAGS='--deny warnings' cargo doc --locked --no-deps --document-private-items

cargo clippy

rm -f snapshot.txt

cargo run foo snapshot.txt && ( echo "ERROR: Unexpected success" && exit 1 )

UPDATE_SNAPSHOTS=1 cargo run foo snapshot.txt || ( echo "ERROR: Expected failure" && exit 1 )

cargo run foo snapshot.txt || ( echo "ERROR: Expected failure" && exit 1 )

cargo run bar snapshot.txt && ( echo "ERROR: Unexpected success" && exit 1 )

UPDATE_SNAPSHOTS=1 cargo run bar snapshot.txt && ( echo "ERROR: Unexpected success" && exit 1 )

cargo run bar snapshot.txt || ( echo "ERROR: Expected failure" && exit 1 )
