#!/usr/bin/env bash
set -euo pipefail
cargo test -p shield-property-tests
cargo test -p shield-integration-suites
cargo test -p shield-test-fixtures
