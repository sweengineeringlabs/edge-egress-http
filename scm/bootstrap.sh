#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "==> Installing git hooks"
git -C "$REPO_ROOT" config core.hooksPath scm/scripts/hooks
echo "==> Fetching dependencies"
(cd "$REPO_ROOT/scm" && cargo fetch --locked)
echo "==> Creating src/gateway symlinks for SEA audit compliance"
for crate in auth breaker cache cassette oauth rate retry tls transport; do
  src_dir="$REPO_ROOT/scm/$crate/src"
  target="$REPO_ROOT/scm/$crate/main/src/gateway"
  mkdir -p "$src_dir"
  if [ ! -e "$src_dir/gateway" ]; then
    ln -s "$target" "$src_dir/gateway"
  fi
done
echo "Bootstrap complete."
