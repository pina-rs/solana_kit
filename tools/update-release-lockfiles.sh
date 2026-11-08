#!/usr/bin/env bash
# Refresh every standalone Cargo.lock that names a workspace crate without
# upgrading unrelated dependencies. `cargo metadata` rewrites only the lockfile
# entries the current manifests require, so the release keeps the reviewed
# third-party versions.
set -euo pipefail

while IFS= read -r -d '' lock; do
  directory="${lock%/Cargo.lock}"
  if [ ! -f "$directory/Cargo.toml" ]; then
    continue
  fi
  cargo metadata --format-version 1 --manifest-path "$directory/Cargo.toml" >/dev/null
done < <(find . -name Cargo.lock -not -path './target/*' -not -path '*/target/*' -print0)
