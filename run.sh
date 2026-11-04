#!/usr/bin/env bash
# Measures every cell with the harness itself. Exits 1 if a verdict is wrong.
# Header of a cell: "// rule: X kind: Y", then "// reason: text" when it must stop.
#   control    must pass
#   violation  must stop, with a message that contains the reason
#   gap        should stop but has no rule yet, so it passes today
# The harness also audits its own source, and that audit counts as one more verdict.
set -u
root=$(cd "$(dirname "$0")" && pwd)
cd "$root" || exit 2
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
cargo build --quiet || exit 2
harness="$root/target/debug/rust-harness"
wrong=0
echo "| Cell | Rule | Kind | Result | Verdict | Message |"
echo "|---|---|---|---|---|---|"
for cell in cells/*.rs; do
  name=$(basename "$cell" .rs)
  rule=$(sed -n '1s/.*rule: *\([^ ]*\).*/\1/p' "$cell")
  kind=$(sed -n '1s/.*kind: *\([^ ]*\).*/\1/p' "$cell")
  reason=$(sed -n '2s/^\/\/ reason: *//p' "$cell")
  "$harness" "$cell" >"$tmp/log" 2>&1
  status=$?
  message=$(grep -E '^error' "$tmp/log" | grep -v 'aborting' | head -1 | sed -E 's/^error(\[[A-Z0-9]+\])?: //; s/\|/\\|/g' | cut -c1-70)
  all=$(grep -E '^error' "$tmp/log")
  case $status in 0) result=passes ;; 1) result=stops ;; *) result=error ;; esac
  case "$kind:$result" in
    control:passes|gap:passes) verdict=ok ;;
    violation:stops) if [ -n "$reason" ] && grep -qF -- "$reason" <<<"$all"; then verdict=ok; else verdict='**REASON**'; fi ;;
    gap:stops) verdict='**REVIEW**' ;;
    *) verdict='**FAIL**' ;;
  esac
  [ "$verdict" = ok ] || wrong=$((wrong+1))
  echo "| \`$name\` | $rule | $kind | $result | $verdict | $message |"
done
echo
if "$harness" --exclude cells . >"$tmp/self" 2>&1; then
  echo "Self-audit: ok. $(tail -1 "$tmp/self")"
else
  echo "Self-audit: **FAIL**"; cat "$tmp/self"; wrong=$((wrong+1))
fi
echo "$(clippy-driver --version | head -1). Wrong verdicts: $wrong."
[ "$wrong" -eq 0 ]
