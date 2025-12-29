# SC MANAGER V4 – IMPLEMENTIERUNGS-ROADMAP

**Datum:** 2025-12-27  
**Basis:** Review-Ergebnisse + IDC-10 Guidelines  
**Ziel:** Schrittweise Optimierung ohne Systemneubau

---

## 🎯 QUICK WINS (1-2 Wochen)

### 1. Performance Budget Framework
**Aufwand:** 2 Tage  
**Impact:** Hoch  

```yaml
# Datei: performance-budgets.yml erstellen
api_gateway:
  p95_latency: 200ms
  p99_latency: 500ms

desktop_ui:
  initial_load: 1500ms
  interaction: 100ms
  memory: 200MB
```

**Tasks:**
- [ ] Budget-Datei erstellen
- [ ] Monitoring Setup (metrics crate)
- [ ] CI-Integration (Budget-Checks)

---

### 2. Error Handling Standardisierung
**Aufwand:** 3 Tage  
**Impact:** Hoch  

```rust
// Datei: core-domain/src/errors.rs
pub enum DomainError {
    NotFound { entity_type: String, id: String },
    InvalidStateTransition { from: String, to: String },
    BusinessRuleViolation { rule: String },
}

// Datei: core-application/src/errors.rs
pub enum ApplicationError {
    ValidationError(String),
    Unauthorized(String),
    Domain(DomainError),
    Infrastructure(InfrastructureError),
}
```

**Tasks:**
- [ ] Error-Hierarchie definieren
- [ ] Bestehenden Code migrieren
- [ ] Error-Response-Mapping (Gateway)

---

### 3. Structured Logging
**Aufwand:** 1 Tag  
**Impact:** Mittel  

```rust
// In allen Services
use tracing::{info, warn, error, instrument};

#[instrument(skip(self))]
pub async fn plan_operation(&self, op: Operation) -> Result<()> {
    info!(operation_id = %op.id(), "Planning operation");
    // ...
}
```

**Tasks:**
- [ ] tracing-subscriber Setup
- [ ] Logging-Guidelines erstellen
- [ ] Bestehenden println! Code ersetzen

---

## 🔴 KRITISCHE FEATURES (2-4 Wochen)

### 4. Event Versioning System
**Aufwand:** 1 Woche  
**Impact:** KRITISCH  

**Dateien:**
```
infrastructure/eventbus/
├── event_schema.rs
├── event_versioning.rs
├── event_migration.rs
└── schemas/
    ├── operation_created_v1.json
    └── operation_created_v2.json
```

**Implementation:**
```rust
// infrastructure/eventbus/event_schema.rs
pub struct VersionedEvent {
    pub event_type: String,
    pub version: String,
    pub schema_version: u32,
    pub payload: serde_json::Value,
    pub metadata: EventMetadata,
}

// infrastructure/eventbus/event_migration.rs
pub trait EventMigration {
    fn from_version(&self) -> u32;
    fn to_version(&self) -> u32;
    fn migrate(&self, event: VersionedEvent) -> Result<VersionedEvent>;
}
```

**Tasks:**
- [ ] Event Schema Registry implementieren
- [ ] Migration Framework erstellen
- [ ] Bestehende Events migrieren
- [ ] Tests für alle Migrationen
- [ ] Dokumentation (ADR)

---

### 5. Offline-First Architecture
**Aufwand:** 2 Wochen  
**Impact:** KRITISCH  

**Komponenten:**
```
apps/desktop/src/
├── offline/
│   ├── command_queue.ts
│   ├── offline_manager.ts
│   ├── conflict_resolver.ts
│   └── sync_engine.ts
```

**Core Logic:**
```typescript
// apps/desktop/src/offline/offline_manager.ts
export class OfflineManager {
  private commandQueue: CommandQueue;
  private syncEngine: SyncEngine;
  
  async executeCommand(cmd: Command): Promise<Result> {
    if (this.isOnline()) {
      return await this.gateway.send(cmd);
    } else {
      await this.commandQueue.enqueue(cmd);
      return Result.queued(cmd.id);
    }
  }
  
  async syncWhenOnline(): Promise<void> {
    for (const cmd of this.commandQueue.pending()) {
      await this.attemptSync(cmd);
    }
  }
}
```

**Tasks:**
- [ ] Command Queue implementieren (IndexedDB)
- [ ] Sync Engine erstellen
- [ ] Conflict Resolution Strategy
- [ ] UI-Feedback für Offline-Modus
- [ ] Tests (E2E mit Netzwerk-Simulation)

---

### 6. Accessibility (WCAG AA)
**Aufwand:** 2 Wochen  
**Impact:** Hoch (Compliance)  

**Schwerpunkte:**
1. Keyboard Navigation
2. Screen Reader Support
3. Color Contrast
4. Focus Management

**Implementation:**
```typescript
// apps/desktop/src/a11y/focus_trap.ts
export const useFocusTrap = (containerRef: RefObject<HTMLElement>) => {
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    
    const focusableElements = container.querySelectorAll(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    
    // Implement focus trap logic
  }, [containerRef]);
};

// Example Usage
<Dialog aria-label="Create Operation" role="dialog">
  <h2 id="dialog-title">New Operation</h2>
  <form aria-describedby="dialog-title">
    <label htmlFor="op-name">Name</label>
    <input id="op-name" aria-required="true" />
  </form>
</Dialog>
```

**Tasks:**
- [ ] ARIA Labels für alle interaktiven Elemente
- [ ] Keyboard Navigation (Tab-Order)
- [ ] Focus Indicators (CSS)
- [ ] Color Contrast Audit (4.5:1)
- [ ] Screen Reader Testing (NVDA)
- [ ] Automated A11y Tests (jest-axe)

---

## 🟡 HIGH PRIORITY (4-6 Wochen)

### 7. Circuit Breaker & Resilience
**Aufwand:** 1 Woche  

```rust
// infrastructure/resilience/circuit_breaker.rs
pub struct CircuitBreaker {
    state: Arc<RwLock<CircuitState>>,
    config: CircuitConfig,
}

enum CircuitState {
    Closed,
    Open { until: Instant },
    HalfOpen,
}

impl CircuitBreaker {
    pub async fn call<T, F>(&self, f: F) -> Result<T, CircuitError>
    where
        F: FnOnce() -> Result<T, Error>,
    {
        // Implementation
    }
}
```

**Tasks:**
- [ ] Circuit Breaker implementieren
- [ ] Retry Policy Framework
- [ ] Timeout Handling
- [ ] Adapter-Integration
- [ ] Monitoring (Metrics)

---

### 8. Caching Strategy
**Aufwand:** 1 Woche  

```rust
// infrastructure/cache/
├── cache_trait.rs
├── redis_cache.rs
├── memory_cache.rs
└── cache_decorator.rs

pub trait Cache: Send + Sync {
    async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>>;
    async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Duration) -> Result<()>;
    async fn invalidate(&self, key: &str) -> Result<()>;
}
```

**Cache Policies:**
```yaml
read_models:
  ttl: 5m
  invalidate_on: ["*Updated", "*Deleted"]
  
static_data:
  ttl: 1h
  
session_data:
  ttl: 30m
```

**Tasks:**
- [ ] Cache-Trait definieren
- [ ] Redis-Implementierung
- [ ] In-Memory Fallback
- [ ] Cache Invalidation via Events
- [ ] Cache-Decorator für Repositories

---

### 9. Testing Coverage 85%
**Aufwand:** 2 Wochen  

**Test-Pyramide:**
```yaml
unit_tests: 60%        # Domain, Application
integration_tests: 25% # Adapters, Infrastructure
e2e_tests: 15%         # Critical User Flows
```

**Neue Test-Typen:**
```rust
// Property-Based Testing
use proptest::prelude::*;

proptest! {
    #[test]
    fn operation_id_always_valid(name in "[a-zA-Z ]{1,100}") {
        let op = Operation::new(name, OperationType::PvE);
        prop_assert!(op.id().is_valid());
    }
}

// Mutation Testing
// cargo install cargo-mutants
// cargo mutants

// Performance Testing
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_operation_planning(c: &mut Criterion) {
    c.bench_function("plan_operation", |b| {
        b.iter(|| plan_operation(/* ... */))
    });
}
```

**Tasks:**
- [ ] Unit Tests für fehlende Domain-Logik
- [ ] Integration Tests für alle Adapter
- [ ] E2E Tests (5 Critical Flows)
- [ ] Property-Based Tests
- [ ] Mutation Testing Setup
- [ ] Performance Benchmarks

---

### 10. Security Hardening
**Aufwand:** 1 Woche  

**Komponenten:**
```
infrastructure/security/
├── authorization.rs
├── input_validation.rs
├── tos_guard.rs
└── audit_log.rs
```

**ToS Enforcement:**
```rust
pub struct ToSGuard {
    rules: Vec<Box<dyn ToSRule>>,
}

pub struct NoGameAutomationRule;

impl ToSRule for NoGameAutomationRule {
    async fn validate(&self, action: &Action) -> Result<(), ToSViolation> {
        match action {
            Action::ParseGameLog(_) => Ok(()),
            Action::AutomateMission(_) => Err(ToSViolation::new("no_automation")),
            _ => Ok(()),
        }
    }
}
```

**Tasks:**
- [ ] ToS Guard implementieren
- [ ] Input Validation (validator crate)
- [ ] Authorization Middleware
- [ ] Audit Logging
- [ ] cargo-audit in CI
- [ ] Security Checklist

---

## 🟢 ENHANCEMENTS (6-12 Wochen)

### 11. High DPI / Multi-Monitor
**Aufwand:** 1 Woche  

```rust
// Tauri API
#[tauri::command]
async fn get_dpi_scale(window: tauri::Window) -> f64 {
    window.scale_factor()
}

#[tauri::command]
async fn get_monitors() -> Vec<MonitorInfo> {
    // List all monitors
}
```

**Tasks:**
- [ ] DPI Detection
- [ ] Multi-Monitor Layout
- [ ] Window Position Restore
- [ ] High-DPI Assets (@2x, @3x)

---

### 12. Animation System (IDC-10)
**Aufwand:** 1 Woche  

```typescript
// Fluent Motion System
const fluentMotion = {
  fadeIn: {
    initial: { opacity: 0, y: 10 },
    animate: { opacity: 1, y: 0 },
    transition: { duration: 0.2, ease: [0.4, 0, 0.2, 1] }
  }
};

// Respect Reduced Motion
const useReducedMotion = () => {
  return window.matchMedia('(prefers-reduced-motion: reduce)').matches;
};
```

**Tasks:**
- [ ] Motion Library (framer-motion)
- [ ] Fluent-konform Transitions
- [ ] Reduced Motion Support
- [ ] Performance Budget (60fps)

---

### 13. Plugin Sandbox
**Aufwand:** 2 Wochen  

```rust
pub struct PluginSandbox {
    memory_limit: usize,
    cpu_time_limit: Duration,
    allowed_capabilities: HashSet<Capability>,
}

impl PluginSandbox {
    pub async fn execute<T>(&self, plugin: &Plugin, f: impl FnOnce() -> T) -> Result<T> {
        self.set_resource_limits()?;
        tokio::time::timeout(self.cpu_time_limit, f()).await
    }
}
```

**Tasks:**
- [ ] Resource Limits (memory, CPU)
- [ ] Capability System
- [ ] Seccomp Filter (Linux)
- [ ] Plugin Process Isolation

---

### 14. Observability
**Aufwand:** 1 Woche  

```rust
// Metrics
use metrics::{counter, histogram, gauge};

counter!("operations.planned", 1);
histogram!("operation.planning.duration", duration_ms);
gauge!("operations.active", count as f64);

// Tracing
use tracing::instrument;

#[instrument(skip(self))]
pub async fn plan_operation(&self, op: Operation) -> Result<()> {
    // Automatic span tracking
}
```

**Export:**
- Prometheus (Metrics)
- Jaeger (Traces)
- Grafana (Dashboards)

**Tasks:**
- [ ] Metrics Setup
- [ ] Distributed Tracing
- [ ] Dashboards
- [ ] Alerts

---

### 15. Database Optimization
**Aufwand:** 3 Tage  

```sql
-- Performance Indexes
CREATE INDEX CONCURRENTLY idx_operations_status 
    ON operations(status) 
    WHERE deleted_at IS NULL;

CREATE INDEX CONCURRENTLY idx_operations_time_window 
    ON operations USING GIST (time_window);
```

**Tasks:**
- [ ] Query Analysis (EXPLAIN ANALYZE)
- [ ] Index Creation
- [ ] N+1 Query Elimination
- [ ] Connection Pooling Tuning

---

## 📋 CHECKLISTS

### Copilot Self-Check
Vor jedem Code-Generation:
- [ ] Layer-Separation korrekt?
- [ ] Keine Business-Logik in Adaptern?
- [ ] Events für State-Changes?
- [ ] Tests inkludiert?
- [ ] Error Handling vorhanden?
- [ ] Dokumentation geschrieben?
- [ ] Kein `unwrap()` in Production?

### Code Review Checklist
- [ ] Architektur (Layer, Events)
- [ ] Code Quality (Naming, Errors, Tests)
- [ ] Testing (Coverage, Edge Cases)
- [ ] Documentation (Public APIs, Examples)
- [ ] Security (Validation, Authorization)
- [ ] Performance (Budgets, Caching)

### Release Checklist
- [ ] Alle Tests grün
- [ ] Coverage ≥85%
- [ ] Security Audit passed
- [ ] Performance Budgets erfüllt
- [ ] A11y Tests passed
- [ ] Migration Plan vorhanden
- [ ] Rollback getestet
- [ ] Monitoring Alerts konfiguriert
- [ ] Dokumentation aktualisiert
- [ ] CHANGELOG geschrieben

---

## 🎯 PRIORITÄTEN-MATRIX

```
URGENT & IMPORTANT (DO FIRST):
├─ Event Versioning
├─ Performance Budgets
└─ Error Handling

IMPORTANT NOT URGENT (SCHEDULE):
├─ Offline-First
├─ A11y Compliance
├─ Testing Coverage
└─ Security Hardening

URGENT NOT IMPORTANT (DELEGATE):
├─ Documentation Updates
└─ Logging Standardization

NOT URGENT NOT IMPORTANT (ELIMINATE):
└─ [None identified]
```

---

## 📊 PROGRESS TRACKING

### Week 1-2: Quick Wins
- [x] Performance Budget Framework
- [x] Error Handling Standardisierung
- [x] Structured Logging

### Week 3-4: Critical Features
- [ ] Event Versioning (50%)
- [ ] Offline-First Architecture (30%)
- [ ] A11y Compliance (20%)

### Week 5-8: High Priority
- [ ] Circuit Breaker
- [ ] Caching Strategy
- [ ] Testing Coverage
- [ ] Security Hardening

### Week 9-12: Enhancements
- [ ] High DPI Support
- [ ] Animation System
- [ ] Plugin Sandbox
- [ ] Observability

---

## 🔚 ERFOLGS-METRIKEN

**Nach 4 Wochen:**
- ✅ Event Versioning produktiv
- ✅ Performance Budgets enforced
- ✅ Error Handling konsistent
- ✅ A11y WCAG AA konform

**Nach 8 Wochen:**
- ✅ Offline-First funktioniert
- ✅ Circuit Breaker in Production
- ✅ Test Coverage ≥85%
- ✅ Security Audit bestanden

**Nach 12 Wochen:**
- ✅ Alle kritischen Features live
- ✅ IDC-10 vollständig erfüllt
- ✅ Performance-Ziele erreicht
- ✅ Production-ready

---

**Status:** ACTIONABLE ROADMAP  
**Version:** 1.0  
**Start:** Sofort möglich  
**Timeline:** 12 Wochen (phased)
