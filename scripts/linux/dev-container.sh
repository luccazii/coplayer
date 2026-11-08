#!/usr/bin/env bash
set -eu
project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$project_dir"
# Reuse the toolchain pinned and installed for this project.
exec distrobox enter coplayer-dev -- bash -c 'source .local/tools/env.sh; export CARGO_BUILD_JOBS=4 APPIMAGE_EXTRACT_AND_RUN=1; exec "$@"' bash "$@"
