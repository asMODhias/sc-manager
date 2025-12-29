---
title: COPILOT_V6_ULTIMATE_SUPPLEMENT
version: 6.1.0
extends: COPILOT_MASTER_INSTRUCTION_V6_COMPLETE_WITH_PLUGINS.md
date: 2025-12-27
---

# 🎯 V6 ULTIMATE SUPPLEMENT

**Dieses Dokument ERWEITERT V6 Complete um:**
- Local-First Development
- Zero-Questions Policy
- Automated Decision Making
- Complete CI/CD Local

---

## 🚨 CRITICAL: NO QUESTIONS POLICY

```yaml
Copilot_Behavior:
  Questions: FORBIDDEN
  Ambiguity: RESOLVE_AUTOMATICALLY
  Uncertainty: USE_DECISION_TREES
  
Forbidden_Phrases:
  - "Soll ich...?"
  - "Möchtest du...?"
  - "Welche Variante...?"
  - "Wie soll ich...?"
  - "Was bevorzugst du...?"

Instead:
  1. Read base document (V6 Complete)
  2. Use decision trees below
  3. Apply default choices
  4. Generate code immediately
  5. Let local CI validate
```

---

## 🏠 LOCAL-FIRST DEVELOPMENT

### Development Workflow

```bash
# 1. Code locally
vim src/domain/operation.rs

# 2. Test locally
cargo test
pnpm test

# 3. Run local CI
./scripts/run-local-ci.sh

# 4. Push only when green
git push origin feature/xyz

# Remote is MIRROR only
```

### Local CI Setup

```bash
# Install act (nektos/act)
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Start local services
docker-compose up -d

# Run CI locally
act -j test
act -j security
act -j build

# Full pipeline
act
```

### Local CI Configuration

```yaml
# .github/workflows/local-ci.yml
name: Local CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16-alpine
        env:
          POSTGRES_PASSWORD: test
        ports:
          - 5432:5432
      dragonflydb:
        image: docker.dragonflydb.io/dragonflydb/dragonfly
        ports:
          - 6379:6379
    
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Test All
        run: |
          cargo test --all-features
          pnpm test
      - name: Coverage
        run: |
          cargo tarpaulin --all-features --out Xml
          python3 -c "
          import xml.etree.ElementTree as ET
          tree = ET.parse('cobertura.xml')
          cov = float(tree.getroot().attrib['line-rate']) * 100
          assert cov >= 85, f'Coverage {cov:.1f}% < 85%'
          "
```

---

## 🎯 DECISION TREES (EXTENDED)

### 1. Data Structure Selection

```
Need ordering?
├─ YES → BTreeMap / BTreeSet
└─ NO
   └─ Need fast lookup?
      ├─ YES → HashMap / HashSet
      └─ NO → Vec

Default: HashMap (most common case)
```

### 2. Layer Assignment

```
Is business rule?
├─ YES → Domain Layer
└─ NO
   └─ Orchestrates domain?
      ├─ YES → Application Layer
      └─ NO
         └─ Calls external system?
            ├─ YES → Adapter Layer
            └─ NO → Infrastructure Layer
```

### 3. Error Type Selection

```
Domain rule violation?
├─ YES → DomainError
└─ NO
   └─ Application validation?
      ├─ YES → ApplicationError::ValidationError
      └─ NO
         └─ External failure?
            ├─ YES → InfrastructureError
            └─ NO → PluginError (if plugin context)
```

### 4. Async Decision

```
Calls I/O?
├─ YES → async fn
└─ NO
   └─ Pure computation?
      ├─ YES → sync fn
      └─ NO → async fn (if spawns tasks)

Examples:
- Database: async
- Business rule: sync
- Event publish: async
- Calculate sum: sync
```

### 5. Test Coverage

```
Component Type?
├─ Domain → 100% (ALL business logic)
├─ Application → 95% (handlers)
├─ Adapter → 85% (external)
├─ Plugin → 85% (plugin code)
└─ UI → 75% (components)

Default: Target from DoD
```

### 6. Retry Strategy

```
Target?
├─ External API
│  ├─ max_retries: 3
│  ├─ backoff: exponential (100ms → 200ms → 400ms)
│  └─ timeout: 5s
├─ Database
│  ├─ max_retries: 2
│  ├─ backoff: linear (100ms → 200ms)
│  └─ timeout: 10s
└─ Internal
   ├─ max_retries: 0
   └─ timeout: 30s
```

### 7. Cache TTL

```
Data Type?
├─ Static (ship specs)
│  └─ ttl: 1 hour
├─ Read models
│  ├─ ttl: 5 minutes
│  └─ invalidate_on_event: true
├─ Real-time
│  └─ no_cache: true
└─ Session
   └─ ttl: 30 minutes
```

### 8. Naming Convention

```
Language?
├─ Rust
│  ├─ module: snake_case
│  ├─ struct: PascalCase
│  ├─ function: snake_case
│  └─ constant: SCREAMING_SNAKE_CASE
└─ TypeScript
   ├─ file: kebab-case
   ├─ component: PascalCase
   ├─ function: camelCase
   └─ constant: SCREAMING_SNAKE_CASE
```

### 9. Grinding ToS Check

```
Feature Action?
├─ Automates gameplay? → ❌ FORBIDDEN
├─ Auto-completes mission? → ❌ FORBIDDEN
├─ Writes to RSI API? → ❌ FORBIDDEN
├─ Bot farming? → ❌ FORBIDDEN
└─ Manual + Verification?
   └─ ✅ ALLOWED
```

---

## 📋 DEFAULT CHOICES

```yaml
When_not_specified:
  data_structure: HashMap
  error_library: thiserror
  logging: tracing
  async_runtime: tokio
  retry_count: 3
  cache_ttl: 5min
  timeout: 5s
  test_coverage: 85%
  
Always:
  no_unwrap: true
  no_panic: true
  no_expect: true
  strict_typescript: true
  
Performance:
  optimize_after: correctness_proven
  premature_optimization: forbidden
  measure_before: optimizing
```

---

## ⚡ LOCAL CI SCRIPTS

### run-local-ci.sh

```bash
#!/bin/bash
set -e

echo "🚀 Local CI Pipeline Starting..."

# Start services
docker-compose up -d
sleep 5

# Format check
echo "📝 Format check..."
cargo fmt --all -- --check
pnpm biome check .

# Lint
echo "🔍 Lint check..."
cargo clippy --all-targets -- -D warnings

# Tests
echo "🧪 Running tests..."
cargo test --all-features --workspace
pnpm test
pnpm --filter './plugins/*' test

# Coverage
echo "📊 Coverage check..."
cargo tarpaulin --all-features --out Xml
python3 << EOF
import xml.etree.ElementTree as ET
tree = ET.parse('cobertura.xml')
cov = float(tree.getroot().attrib['line-rate']) * 100
print(f"Coverage: {cov:.1f}%")
if cov < 85:
    print(f"❌ Coverage below 85%")
    exit(1)
print("✅ Coverage OK")
EOF

# Security
echo "🔒 Security audit..."
cargo audit
cargo deny check

# Build
echo "🏗️ Building..."
pnpm turbo build

echo "✅ Local CI passed!"
```

### test-local.sh (Quick check)

```bash
#!/bin/bash
set -e

# Quick tests only (no coverage, no build)
cargo test --lib
pnpm test --run

echo "✅ Quick tests passed!"
```

---

## 🎮 USAGE EXAMPLES

### Example 1: Adding New Feature

```bash
# 1. Create branch
git checkout -b feature/new-feature

# 2. Code (Copilot generates)
# - Reads V6 Complete base document
# - Uses decision trees for uncertainties
# - NO questions asked
# - Generates complete code with tests

# 3. Test locally
./scripts/test-local.sh

# 4. Run local CI
./scripts/run-local-ci.sh

# 5. Commit
git commit -am "feat: new feature"

# 6. Push (only when CI green)
git push origin feature/new-feature
```

### Example 2: Copilot Uncertain

```
Copilot thinks: "Should I use HashMap or BTreeMap?"

OLD Behavior (V6 Complete):
❌ "Möchtest du HashMap oder BTreeMap?"

NEW Behavior (V6 Ultimate):
✅ Check decision tree → Need ordering? NO → Use HashMap
✅ Generate code with HashMap
✅ Add comment: // Using HashMap (unordered collection)
✅ Continue without asking
```

### Example 3: Plugin Development

```bash
# 1. Create plugin from template
cp -r plugins/_template plugins/my-plugin

# 2. Copilot generates (automatically):
# - plugin.json (correct schema)
# - index.ts (implements Plugin interface)
# - domain/ (plugin domain model)
# - ui/ (SolidJS components)
# - tests/ (85% coverage target)

# 3. Test plugin
pnpm --filter my-plugin test

# 4. Build plugin
pnpm --filter my-plugin build

# 5. Load in app (automatic via registry)
```

---

## 🔐 SECURITY HARDENING

### Automated Security Checks

```yaml
Pre-commit:
  - cargo audit
  - cargo deny check licenses
  - cargo deny check advisories
  - grep -r "unwrap()" src/ && exit 1
  - grep -r "panic!()" src/ && exit 1

Weekly:
  - cargo outdated
  - cargo mutants (mutation testing)
  - OWASP dependency check

Release:
  - Full security audit
  - Penetration testing
  - Code signing
```

---

## 📊 METRICS & MONITORING

### CI Metrics

```yaml
Track:
  - Build time (target: <5min)
  - Test execution time (target: <2min)
  - Coverage % (target: ≥85%)
  - Security issues (target: 0)
  - Performance budgets (enforce)

Report:
  - Per commit
  - Trends over time
  - Regression detection
```

---

## 🎯 COPILOT V6 ULTIMATE SUMMARY

```
BASE: V6 Complete (58KB)
  ↓
ADDS: V6 Ultimate Supplement (this file)
  ↓
RESULT: V6 Ultimate (Complete + Zero-Questions + Local-First)

Key Differences:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
V6 Complete          V6 Ultimate
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
May ask questions    NEVER asks questions
Remote CI optional   Local CI mandatory
General guidance     Decision trees
Examples provided    Deterministic rules
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Usage:
1. Read V6 Complete (base knowledge)
2. Apply V6 Ultimate (behavior rules)
3. Generate code (no questions)
4. Test locally (CI enforced)
5. Push when green (remote mirror)
```

---

## ✅ FINAL CHECKLIST

```yaml
Before_using_Copilot:
  - [x] V6 Complete document read
  - [x] V6 Ultimate supplement read
  - [x] Local environment setup (Docker, act, pnpm)
  - [x] Decision trees understood
  - [x] Default choices noted

While_Copilot_generates:
  - [x] Never asks questions
  - [x] Uses decision trees
  - [x] Applies defaults
  - [x] Includes tests
  - [x] Production-quality code

After_generation:
  - [x] Run local tests
  - [x] Run local CI
  - [x] All checks green
  - [x] Coverage ≥85%
  - [x] No security issues
  - [x] Push to remote
```

---

**STATUS:** V6 ULTIMATE READY  
**BASE:** V6 Complete (58KB)  
**SUPPLEMENT:** This file  
**QUESTIONS:** ZERO  
**LOCAL-FIRST:** YES  
**DETERMINISTIC:** ABSOLUTE  
**PRODUCTION-READY:** YES

