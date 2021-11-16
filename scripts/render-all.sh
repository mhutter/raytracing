#!/usr/bin/env bash
set -e -u -o pipefail

readme="target/README.md"
echo "# Progress" > "$readme"

n=0
git stash
git rev-list main | \
tail -r | \
while read -r rev; do
  printf -v ppm 'target/%02d_%s.ppm' $n "${rev:0:7}"
  printf -v png '%02d_%s.png' $n "${rev:0:7}"
  printf -v entry '\n## %02d %s\n\n![%s](%s)' \
    $n \
    "$(git log -n1 --pretty=format:%s "$rev")" \
    "$rev" \
    "$png"

  echo "$entry" >> "$readme"

  n=$((n+1))
  [ -f "$ppm" ] || {
    git checkout --quiet "$rev"
    cargo run --release > "$ppm"
  }

  [ -f "progress/$png" ] || convert "$ppm" "progress/$png"
done

git checkout --quiet main
git stash pop
