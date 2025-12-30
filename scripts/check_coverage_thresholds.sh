#!/usr/bin/env bash
set -euo pipefail

# thresholds (as fractions)
DOMAIN_THRESH=1.00
APP_THRESH=0.95
ADAPTERS_THRESH=0.85
OVERALL_THRESH=0.85

# helper: extract line-rate from cobertura or tarpaulin xml
extract_rate() {
  f=$1
  if [ ! -f "$f" ]; then
    echo "0"
    return
  fi
  # line-rate can appear as line-rate="0.123" or line-rate="0.123"
  rate=$(grep -o 'line-rate="[0-9.eE+-]*"' "$f" | head -n1 | sed -E 's/line-rate="([0-9.]+)"/\1/')
  if [ -z "$rate" ]; then
    echo "0"
  else
    echo "$rate"
  fi
}

# collect crate rates
declare -A RATES
for dir in artifacts/coverage/*; do
  if [ -d "$dir" ]; then
    crate=$(basename "$dir")
    # try common xml names
    if [ -f "$dir/cobertura.xml" ]; then
      xml="$dir/cobertura.xml"
    elif [ -f "$dir/tarpaulin-report.xml" ]; then
      xml="$dir/tarpaulin-report.xml"
    elif [ -f "$dir/${crate}-tarpaulin.xml" ]; then
      xml="$dir/${crate}-tarpaulin.xml"
    else
      # fallback to any xml
      xml=$(ls "$dir"/*.xml 2>/dev/null | head -n1 || true)
    fi
    rate=$(extract_rate "$xml")
    RATES[$crate]=$rate
    echo "crate=$crate rate=$rate"
  fi
done

# grouping
domain_crates=("core" "services-core-domain")
app_crates=("app" "core-application")
adapters_crates=("adapters" "adapter-p2p")

fail=0
# check group averages/mins
check_group() {
  name=$1
  thresh=$2
  shift 2
  crates=("$@")
  sum=0
  count=0
  for c in "${crates[@]}"; do
    r=${RATES[$c]:-0}
    sum=$(awk "BEGIN { printf \"%f\" , $sum + $r }")
    count=$((count+1))
    if awk "BEGIN{exit !($r >= $thresh)}"; then
      :
    else
      echo "FAIL: crate $c in group $name below threshold: $r < $thresh"
      fail=1
    fi
  done
  avg=$(awk "BEGIN { printf \"%f\" , $sum / $count }")
  echo "Group $name average: $avg (threshold $thresh)"
}

check_group domain $DOMAIN_THRESH "${domain_crates[@]}"
check_group application $APP_THRESH "${app_crates[@]}"
check_group adapters $ADAPTERS_THRESH "${adapters_crates[@]}"

# overall average
sum=0; count=0
for k in "${!RATES[@]}"; do
  sum=$(awk "BEGIN { printf \"%f\" , $sum + ${RATES[$k]} }")
  count=$((count+1))
done
if [ $count -eq 0 ]; then
  echo "No coverage artifacts found"
  exit 1
fi
overall=$(awk "BEGIN { printf \"%f\" , $sum / $count }")
echo "Overall average: $overall (threshold $OVERALL_THRESH)"
if awk "BEGIN{exit !($overall >= $OVERALL_THRESH)}"; then
  :
else
  echo "FAIL: overall coverage below threshold"
  fail=1
fi

if [ $fail -ne 0 ]; then
  echo "Coverage thresholds not met"
  exit 2
fi

echo "Coverage thresholds OK"
