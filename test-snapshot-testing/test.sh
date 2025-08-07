#!/usr/bin/env bash
set -o errexit -o nounset

cargo fmt -- --check

RUSTDOCFLAGS='--deny warnings' cargo doc --locked --no-deps --document-private-items

cargo clippy

