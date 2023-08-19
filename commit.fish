#!/usr/bin/env fish

set CARGO $(env cargo)

$CARGO test --release
git add .
git commit -m "(chore) update types"
git push origin main
