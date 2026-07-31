#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
#
# Verify that every added/modified source file in the range BASE..HEAD
# carries an `SPDX-License-Identifier:` header.
#
# Usage: check-spdx.sh <base-sha> <head-sha>

set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <base-sha> <head-sha>" >&2
    exit 2
fi

base="$1"
head="$2"

status=0
while read -r file; do
    [ -z "$file" ] && continue
    [ -f "$file" ] || continue
    case "$file" in
        *.rs|*.sh) ;;
        *) continue ;;
    esac
    if ! head -n 5 "$file" | grep -q 'SPDX-License-Identifier:'; then
        echo "Missing SPDX-License-Identifier header: ${file}"
        status=1
    fi
done < <(git diff --name-only --diff-filter=ACM "${base}..${head}")

if [ "$status" -ne 0 ]; then
    echo
    echo "Some changed files are missing an SPDX-License-Identifier header."
    exit 1
fi

echo "All changed files have SPDX headers."
