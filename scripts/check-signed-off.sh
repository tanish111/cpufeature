#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
#
# Verify that every commit in the range BASE..HEAD carries a
# `Signed-off-by:` trailer.
#
# Usage: check-signed-off.sh <base-sha> <head-sha>

set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <base-sha> <head-sha>" >&2
    exit 2
fi

base="$1"
head="$2"

status=0
while read -r commit; do
    [ -z "$commit" ] && continue
    subject="$(git log -1 --format='%s' "$commit")"
    if ! git log -1 --format='%B' "$commit" \
        | grep -qiE '^Signed-off-by: .+ <.+@.+>'; then
        echo "Missing Signed-off-by: ${commit:0:12} ${subject}"
        status=1
    fi
done < <(git rev-list "${base}..${head}")

if [ "$status" -ne 0 ]; then
    echo
    echo "Some commits are missing a Signed-off-by line."
    echo "Add one with: git commit --amend --signoff (or --signoff when committing)."
    exit 1
fi

echo "All commits are signed off."
