#!/usr/bin/env bash
# Run every quiz and print a summary.
#
#   ./check.sh              run all quizzes
#   ./check.sh --fail-fast  stop at the first failure
#
set -u
cd "$(dirname "$0")" || exit 1

fail_fast=0
[ "${1:-}" = "--fail-fast" ] && fail_fast=1

passed=0
failed=0
failed_names=()

for file in src/bin/q*.rs; do
    name="$(basename "$file" .rs)"

    if ! output="$(cargo run --quiet --bin "$name" 2>&1)"; then
        printf '❌ %s  (run: cargo run --bin %s)\n' "$name" "$name"
        failed=$((failed + 1))
        failed_names+=("$name")
        if [ "$fail_fast" -eq 1 ]; then
            printf '\n%s\n' "$output"
            exit 1
        fi
        continue
    fi

    if grep -q '✅' <<<"$output"; then
        printf '✅ %s\n' "$name"
        passed=$((passed + 1))
    else
        printf '⚠️  %s finished without a success marker\n' "$name"
        failed=$((failed + 1))
        failed_names+=("$name")
        [ "$fail_fast" -eq 1 ] && exit 1
    fi
done

printf '\n%d passed, %d failed\n' "$passed" "$failed"
if [ "$failed" -gt 0 ]; then
    printf 'Still to solve: %s\n' "${failed_names[*]}"
    exit 1
fi

printf '🎉 All quizzes solved!\n'
