#!/usr/bin/env bash

n=0
git stash
git rev-list main | \
tail -r | \
while read -r rev; do
  printf -v image '%02d_%s.ppm' $n "${rev:0:7}"
  n=$((n+1))
  [ -f "$image" ] && continue
  git checkout --quiet "$rev"
  cargo run --release > "$image"
done

git checkout --quiet main
git stash pop
