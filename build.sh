#!/usr/bin/env bash
set -e

# Local build & test script for Linux.
# Builds debug binaries and runs cram tests.
# Differs from CI:
#   - CI builds release on multiple platforms and packages coreutils tar.gz.
#   - This script only builds debug on the local Linux machine and runs tests.
#
# Usage:
#   ./build.sh         # build + test (default)
#   ./build.sh test    # run tests only, skip cargo build

cd "$(dirname "$0")"

MODE="${1:-all}"

PY=".venv/bin/python"
if [ ! -x "$PY" ]; then
    echo "==> creating venv and installing deps"
    python3 -m venv .venv
    .venv/bin/pip install --quiet -r tests/requirements.txt
fi

if [ "$MODE" = "test" ]; then
    echo "==> skipping cargo build (test mode)"
    export BIN_DIR="$(pwd)/target/debug"

    echo "==> running cram tests"
    "$PY" -m cram --verbose tests/
else
    echo "==> cargo build (debug, all bins)"
    cargo build --bins

    export BIN_DIR="$(pwd)/target/debug"

    echo "==> running cram tests"
    "$PY" -m cram --verbose tests/
fi
