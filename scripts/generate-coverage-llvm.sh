#!/usr/bin/env bash
set -euo pipefail

echo "==> generate-coverage-llvm: looking for profraw files and generating HTML via llvm-profdata/llvm-cov"

# Find profraw files
profraws=()
while IFS= read -r -d $'\0' f; do
  profraws+=("$f")
done < <(find . -name "*.profraw" -print0 || true)

if [ ${#profraws[@]} -eq 0 ]; then
  echo "No .profraw files found; nothing to do."
  exit 0
fi

# Check for llvm-profdata
if ! command -v llvm-profdata >/dev/null 2>&1; then
  echo "llvm-profdata not found in PATH; cannot merge profraws."
  exit 0
fi

mkdir -p artifacts/coverage/llvm

# Merge profraws
echo "Merging profraws into merged.profdata"
llvm-profdata merge -sparse "${profraws[@]}" -o merged.profdata

# Find candidate binaries under target (tests/binaries built by cargo test --no-run)
binaries=()
while IFS= read -r -d $'\0' b; do
  binaries+=("$b")
done < <(find target -type f -perm /a+x -print0 || true)

if [ ${#binaries[@]} -eq 0 ]; then
  echo "No compiled binaries found under target/ to generate coverage for; try running cargo test with coverage instrumentation first."
  exit 0
fi

# Generate HTML per binary
for b in "${binaries[@]}"; do
  name=$(basename "$b")
  echo "Generating HTML coverage for $b -> artifacts/coverage/llvm/$name"
  # llvm-cov may fail for non-instrumented binaries; keep going
  llvm-cov show "$b" -instr-profile=merged.profdata -format=html -output-dir="artifacts/coverage/llvm/$name" || true
  # also export json summary
  llvm-cov export "$b" -instr-profile=merged.profdata > "artifacts/coverage/llvm/${name}.json" || true
done

echo "Coverage generation complete. Artifacts in artifacts/coverage/llvm/"
exit 0
