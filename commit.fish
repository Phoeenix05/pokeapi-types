#!/usr/bin/env fish

cargo test --release
git add .
git commit -m "(chore) update types"
git push origin main
