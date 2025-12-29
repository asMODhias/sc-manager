---
title: COPILOT_MASTER_INSTRUCTION_V5_FINAL_OPTIMIZED
version: 5.0.0
date: 2025-12-27
status: ABSOLUTE - ZERO DEVIATION ALLOWED
priority: MAXIMUM
binding: NON-NEGOTIABLE
---

# 🎯 SC MANAGER V5 - ABSOLUTE COPILOT INSTRUCTION

**ZERO TOLERANCE FOR DEVIATION | COMPLETE FEATURE SET | OPTIMIZED STACK**

---

## 🔴 META-RULES (READ FIRST - ALWAYS)

### Rule 0: Absolute Hierarchy
```
1. THIS DOCUMENT = LAW
2. IDC-10 Guidelines
3. Star Citizen ToS  
4. Best Practices
```
**Conflicts**: Higher level ALWAYS wins.

### Rule 1: Self-Check BEFORE Every Action
```
Copilot MUST verify BEFORE generating ANY code:
✓ Read this document section relevant to task?
✓ Layer separation will be correct?
✓ Using exact tech stack specified?
✓ No business logic going in adapters?
✓ Events used for state changes?
✓ Tests will be included?
✓ Error handling will be present?
✓ No unwrap() in production?
✓ Performance budget respected?
✓ ToS compliance verified?

IF ANY ❌ → STOP and request clarification
```

### Rule 2: Stop Conditions (MANDATORY)
Copilot MUST STOP if:
- Requirement contradicts this document
- No clear implementation path
- Requested feature violates ToS
- Layer separation would break
- Performance budget would exceed
- Security risk identified
- Test coverage would drop

### Rule 3: Forbidden Actions
```yaml
NEVER:
  - Skip tests
  - Merge architectural layers
  - Use unsanctioned tech
  - Add unapproved dependencies
  - Business logic in adapters
  - Direct DB access from UI
  - "Quick fixes" that violate architecture
  - Shortcuts "just this once"
```

---

## 1️⃣ TECH STACK (EXACT - NO SUBSTITUTES)

### Backend (Rust)
```yaml
Language: Rust 1.75+ (edition 2021)
Framework: Axum 0.7+
Database: PostgreSQL 16+
Cache: DragonflyDB 1.13+ (redis-compatible)
Event_Bus: In-Memory (desktop) / NATS JetStream (enterprise)
Serialization: serde + serde_json
ORM: sqlx (compile-time checked)
```

### Desktop (Tauri + SolidJS)
```yaml
Shell: Tauri 2.0+
Frontend: SolidJS 1.8+ (NOT React)
UI_Framework: shadcn/ui + Radix (NOT Fluent UI)
Styling: Tailwind CSS
State: SolidJS Stores (no Redux/MobX)
HTTP_Client: @tanstack/solid-query 5.0+
TypeScript: 5.3+ (strict mode)
```

### Build System
```yaml
Monorepo: Turborepo (latest)
Package_Manager: pnpm 8.0+ (NOT npm)
Rust_Cache: sccache
CI: Local Docker
```

### Development Tools
```yaml
Rust:
  - rustfmt, clippy (mandatory)
  - cargo-audit, cargo-deny
  - cargo-mutants (mutation testing)
  
TypeScript:
  - biome (NOT eslint/prettier)
  - typescript-strict
  
Testing:
  - cargo test, vitest, playwright
  
Monitoring:
  - tracing, metrics, opentelemetry
```

### ⛔ FORBIDDEN Technologies
```
NEVER USE:
- Electron → Use Tauri
- React/Vue/Angular → Use SolidJS  
- Fluent UI → Use shadcn/ui
- npm → Use pnpm
- eslint/prettier → Use biome
- Redux/MobX/Zustand → Use SolidJS stores
- Redis → Use DragonflyDB
- MongoDB/MySQL → Use PostgreSQL
```

---

## 2️⃣ ARCHITECTURE (IMMUTABLE)

### Layer Structure
```
┌─────────────────────────┐
│ UI (SolidJS + shadcn)  │  Commands ↓ | Events ↑
├─────────────────────────┤
│ Application (Rust)      │  Orchestration only
│ - Commands/Queries      │  NO business logic
├─────────────────────────┤
│ Domain (Pure Rust)      │  ALL business logic
│ - Entities/Aggregates   │  ZERO tech dependencies
│ - Value Objects         │
│ - Domain Events         │
├─────────────────────────┤
│ Event Bus               │  Async pub/sub
├─────────────────────────┤
│ Adapters (External)     │  NO business logic
│ - RSI, Discord, etc     │  Read-only transformers
├─────────────────────────┤
│ Infrastructure          │  Tech concerns only
│ - Postgres, Cache, etc  │
└─────────────────────────┘
```

### Layer Rules (ABSOLUTE)

**Domain Layer**
```rust
✅ ALLOWED:
- Business logic (ALL of it)
- Entities, Value Objects, Aggregates
- Domain Events
- Pure functions
- Standard library only

❌ FORBIDDEN:
- async (unless truly concurrent domain logic)
- HTTP, JSON, SQL dependencies
- Infrastructure imports
- Database access
- External API calls
- tokio (except for truly concurrent domain)
```

**Application Layer**
```rust
✅ ALLOWED:
- Command/Query handlers
- Orchestration
- Repository calls
- Event publishing
- Transaction management

❌ FORBIDDEN:
- Business logic (belongs in domain)
- Direct DB queries
- Business validations
- Calculations
```

**Adapter Layer**
```rust
✅ ALLOWED:
- External API calls
- Data transformation
- Event publishing
- Error normalization

❌ FORBIDDEN:
- Business logic
- Business validations
- Data combination
- Decision making
```

### Event-Driven Rules
```yaml
MANDATORY:
  - ALL state changes MUST publish events
  - Events MUST be immutable
  - Events MUST have version + correlation ID
  - UI consumes events ONLY (never polls)
  - No direct component communication
  
EVENT_SCHEMA:
  event_id: UUID
  event_type: string
  version: semver (e.g. "1.0.0")
  schema_version: u32
  timestamp: DateTime<Utc>
  correlation_id: UUID
  causation_id: Option<UUID>
  payload: { ... }
```

---

## 3️⃣ PROJECT STRUCTURE (EXACT)

```
sc-manager/
├── apps/
│   └── desktop/                 # Tauri App
│       ├── src/                 # SolidJS Frontend
│       │   ├── components/      # UI Components (shadcn/ui)
│       │   ├── pages/           # Routes
│       │   ├── stores/          # SolidJS Stores
│       │   └── services/        # API Clients
│       └── src-tauri/           # Rust Backend
│           └── src/
│               ├── commands/    # Tauri Commands
│               └── events/      # Tauri Events
│
├── services/
│   ├── core-domain/             # PURE DOMAIN
│   │   └── src/
│   │       ├── organization/
│   │       ├── operation/
│   │       ├── member/
│   │       ├── fleet/
│   │       └── diplomacy/
│   │
│   ├── core-application/        # USE CASES
│   │   └── src/
│   │       ├── commands/
│   │       ├── queries/
│   │       └── handlers/
│   │
│   └── gateway/                 # API
│       └── src/
│           ├── http/
│           └── websocket/
│
├── adapters/
│   ├── adapter-rsi-auth/
│   ├── adapter-gamelog/
│   ├── adapter-fleetyards/
│   ├── adapter-erkul/
│   ├── adapter-discord/
│   └── adapter-p2p/
│
├── infrastructure/
│   ├── eventbus/
│   ├── persistence/
│   ├── cache/
│   └── resilience/              # Circuit breaker, retry
│
├── tests/
│   ├── unit/
│   ├── integration/
│   ├── e2e/
│   └── performance/
│
├── docs/
│   ├── architecture/
│   │   └── ADR-*.md             # Architecture Decision Records
│   └── api/
│
├── turbo.json                   # Turborepo config
├── pnpm-workspace.yaml
└── Cargo.toml                   # Workspace root
```

---

## 4️⃣ DOMAIN MODEL (COMPLETE)

### Organization Aggregate
```rust
pub struct Organization {
    id: OrganizationId,
    name: OrganizationName,        // Value Object (3-100 chars)
    tag: OrganizationTag,           // Value Object (2-10 alphanumeric)
    divisions: Vec<Division>,
    members: Vec<MemberId>,
    fleets: Vec<FleetId>,
    treasury: Treasury,
    diplomatic_relations: Vec<DiplomaticRelation>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Organization {
    // Business Rules:
    // - Max 1000 members
    // - Unique RSI handles
    // - Cannot establish relations with self
    // - Cannot go Friendly→Hostile directly (must go through Neutral)
    
    pub fn add_member(&mut self, ...) -> Result<DomainEvent, DomainError>
    pub fn plan_operation(&self, ...) -> Result<Operation, DomainError>
    pub fn establish_diplomacy(&mut self, ...) -> Result<DomainEvent, DomainError>
}
```

### Operation Aggregate
```rust
pub struct Operation {
    id: OperationId,
    name: OperationName,
    operation_type: OperationType,  // PvE|PvP|RP|Training|Exploration|Trading|Mining
    organization_id: OrganizationId,
    objective: Objective,
    time_window: TimeWindow,
    status: OperationStatus,        // Planned|Active|Completed|Cancelled
    participants: Vec<Participant>,
    assigned_assets: Vec<AssetAssignment>,
    rules_of_engagement: RulesOfEngagement,
    after_action_report: Option<AfterActionReport>,
}

impl Operation {
    // Business Rules:
    // - Min participants per type (PvE: 2, PvP: 4, etc)
    // - Max participants per type (PvE/PvP: 50, RP: 100, etc)
    // - Time window must be in future
    // - Only one leader per operation
    // - Crew members must be participants
    // - Cannot start outside time window
    // - AAR only after completion
    
    pub fn assign_participant(&mut self, ...) -> Result<DomainEvent, DomainError>
    pub fn assign_asset(&mut self, ...) -> Result<DomainEvent, DomainError>
    pub fn start(&mut self) -> Result<DomainEvent, DomainError>
    pub fn complete(&mut self, ...) -> Result<DomainEvent, DomainError>
    pub fn add_after_action_report(&mut self, ...) -> Result<DomainEvent, DomainError>
}
```

### Member Aggregate
```rust
pub struct Member {
    id: MemberId,
    rsi_identity: RsiIdentity,
    organization_id: OrganizationId,
    roles: Vec<Role>,               // Leader|Officer|Member|Recruit
    qualifications: Vec<Qualification>,
    availability: Availability,     // Available|Away|Busy|Offline
    activity_log: Vec<ActivityEntry>,
    disciplinary_status: DisciplinaryStatus,
    joined_at: DateTime<Utc>,
    last_active_at: DateTime<Utc>,
}

impl Member {
    // Business Rules:
    // - Max 5 roles per member
    // - No duplicate qualifications
    // - Keep last 100 activity entries
    // - Cannot discipline already suspended member
    // - Can only clear warnings/expired suspensions
    
    pub fn assign_role(&mut self, ...) -> Result<DomainEvent, DomainError>
    pub fn add_qualification(&mut self, ...) -> Result<DomainEvent, DomainError>
    pub fn apply_disciplinary_action(&mut self, ...) -> Result<DomainEvent, DomainError>
}
```

### Fleet Aggregate
```rust
pub struct Fleet {
    id: FleetId,
    name: FleetName,
    organization_id: OrganizationId,
    ships: Vec<Ship>,
    readiness_state: ReadinessState,  // FullyReady|MostlyReady|PartiallyReady|NotReady
}

pub struct Ship {
    id: ShipId,
    name: ShipName,
    model: ShipModel,
    status: ShipStatus,              // Operational|Maintenance|Damaged|Destroyed|Impounded
    crew_capacity: CrewCapacity,
    current_crew: Vec<MemberId>,
    location: Location,
    insurance: Insurance,
}

impl Fleet {
    // Business Rules:
    // - No duplicate ships
    // - Auto-update readiness based on operational ships
    //   - 100% operational = FullyReady
    //   - ≥75% operational = MostlyReady
    //   - >0% operational = PartiallyReady
    //   - 0% operational = NotReady
    
    pub fn add_ship(&mut self, ...) -> Result<DomainEvent, DomainError>
    pub fn update_ship_status(&mut self, ...) -> Result<DomainEvent, DomainError>
}
```

### Diplomacy Aggregate
```rust
pub struct DiplomaticRelation {
    id: DiplomaticRelationId,
    organization_a: OrganizationId,
    organization_b: OrganizationId,
    status: DiplomaticStatus,        // Allied|Friendly|Neutral|Unfriendly|Hostile
    agreements: Vec<Agreement>,
    history: Vec<DiplomaticEvent>,
}

impl DiplomaticRelation {
    // Business Rules:
    // - Cannot transition Friendly→Hostile directly
    // - Can only add agreements to Friendly/Allied relations
    
    pub fn update_status(&mut self, ...) -> Result<DomainEvent, DomainError>
    pub fn add_agreement(&mut self, ...) -> Result<DomainEvent, DomainError>
}
```

### Domain Events (Schema)
```rust
pub struct EventMetadata {
    pub event_id: Uuid,
    pub event_type: String,
    pub version: String,              // e.g. "1.0.0"
    pub schema_version: u32,
    pub timestamp: DateTime<Utc>,
    pub correlation_id: Uuid,
    pub causation_id: Option<Uuid>,
    pub aggregate_id: String,
    pub aggregate_type: String,
}

// Events:
// - OrganizationCreated, MemberAdded, MemberRemoved
// - OperationPlanned, OperationStarted, OperationCompleted
// - ParticipantAssigned, AssetAssigned, AfterActionReportAdded
// - RoleAssigned, QualificationAdded, AvailabilityUpdated
// - FleetCreated, ShipAddedToFleet, ShipStatusUpdated
// - DiplomaticRelationEstablished, DiplomaticStatusChanged
// - GameSessionStarted, GameSessionEnded
// - DiscordAccountLinked, DiscordAccountUnlinked
```

---

## 5️⃣ IMPLEMENTATION STANDARDS

### Error Handling (MANDATORY)
```rust
// Three-tier error hierarchy
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },
    
    #[error("Invalid state transition: {from} → {to}")]
    InvalidStateTransition { from: String, to: String },
    
    #[error("Business rule violated: {rule}")]
    BusinessRuleViolation { rule: String },
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Validation failed: {0}")]
    ValidationError(String),
    
    #[error(transparent)]
    Domain(#[from] DomainError),
    
    #[error(transparent)]
    Infrastructure(#[from] InfrastructureError),
}

#[derive(Error, Debug)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("External service unavailable: {service}")]
    ExternalServiceUnavailable { service: String },
}

// ⛔ FORBIDDEN: unwrap(), expect(), panic!() in production
```

### Logging (MANDATORY)
```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(self), fields(
    operation_id = %operation_id,
    participant_count = participants.len()
))]
pub async fn plan_operation(&self, ...) -> Result<()> {
    info!("Planning operation");
    debug!(count = participants.len(), "Validating participants");
    // ...
    info!("Operation planned successfully");
    Ok(())
}

// Levels:
// ERROR: System failure, immediate attention
// WARN:  Degraded operation
// INFO:  Important business events
// DEBUG: Development info
// TRACE: Extremely verbose

// ⛔ FORBIDDEN: println!(), dbg!(), print!()
```

### Type Safety (MANDATORY)
```rust
// ✅ CORRECT: Strong typing
pub struct OperationId(Uuid);
pub struct OrganizationName(String);

// ❌ WRONG: Primitive obsession
pub struct Operation {
    pub id: String,      // ❌
    pub name: String,    // ❌
}
```

### Documentation (MANDATORY)
```rust
/// Creates a new operation.
///
/// # Arguments
/// * `name` - Operation name (3-100 chars)
/// * `op_type` - Type of operation
///
/// # Errors
/// Returns `DomainError::InvalidName` if name invalid
///
/// # Examples
/// ```
/// let op = Operation::new(name, OperationType::PvE)?;
/// ```
pub fn new(...) -> Result<Self, DomainError> {
    // ...
}
```

---

## 6️⃣ TESTING (MANDATORY)

### Coverage Requirements
```yaml
Domain:      100%  # ALL business logic
Application: 95%   # Use cases
Adapters:    85%   # External integration
UI:          75%   # Components
Overall:     85%   # Project minimum

Enforcement:
  - CI blocks merge if below threshold
  - Mutation testing for domain (95% kill rate)
```

### Test Types
```rust
// Unit Test
#[test]
fn operation_requires_minimum_participants() {
    let mut org = create_test_organization();
    let result = org.plan_operation(...);
    assert!(matches!(result.unwrap_err(), DomainError::InsufficientMembers));
}

// Property-Based Test
proptest! {
    #[test]
    fn operation_name_always_validates(name in "[a-zA-Z ]{3,100}") {
        let result = OperationName::try_from(name);
        prop_assert!(result.is_ok());
    }
}

// Integration Test
#[tokio::test]
async fn test_operation_planning_workflow() {
    let pool = test_database().await;
    let handler = PlanOperationHandler::new(...);
    let result = handler.handle(command).await;
    assert!(result.is_ok());
}

// E2E Test (Playwright)
test('user can plan operation', async ({ page }) => {
    await page.goto('/operations');
    await page.click('button:text("Plan Operation")');
    // ...
});
```

---

## 7️⃣ PERFORMANCE BUDGETS (ENFORCED)

```yaml
API_Gateway:
  p50_latency: 50ms
  p95_latency: 200ms
  p99_latency: 500ms

Event_Bus:
  publish_latency: 5ms
  delivery_latency: 50ms

Desktop_UI:
  initial_load: 1500ms
  interaction: 100ms
  memory: 200MB

Adapters:
  rsi_auth: 2000ms
  game_log_parse: 100ms

Monitoring:
  - Metrics via opentelemetry
  - CI fails if budgets exceeded
```

---

## 8️⃣ SECURITY & ToS

### Security Checklist
```yaml
- [ ] No secrets in code
- [ ] Input validation (all user input)
- [ ] SQL injection prevented (sqlx compile-time checks)
- [ ] XSS prevented (sanitization)
- [ ] Authentication checks
- [ ] Authorization (RBAC)
- [ ] Rate limiting
- [ ] cargo-audit in CI (weekly)
```

### ToS Compliance (Star Citizen)
```yaml
ALLOWED:
  - Read Game.log
  - Manual mission reporting
  - Officer verification
  - Reputation tracking
  
FORBIDDEN:
  - Mission automation
  - Gameplay automation
  - RSI API writes
  - Bot-driven farming
  - Auto-completion
  
ToS_Guard: Enforced at adapter layer (technical gate)
```

---

## 9️⃣ DEPLOYMENT

### Build System (Turborepo + pnpm)
```json
// turbo.json
{
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": ["dist/**", "target/**"]
    },
    "test": {
      "dependsOn": ["build"]
    }
  }
}
```

```yaml
# pnpm-workspace.yaml
packages:
  - 'apps/*'
  - 'services/*'
  - 'adapters/*'
```

### CI Pipeline
```yaml
steps:
  - checkout
  - pnpm install
  - cargo audit
  - cargo deny check
  - biome check
  - turbo test
  - turbo build
  - check_coverage (≥85%)
  - check_performance_budgets
  - docker build
```

---

## 🔟 CHECKPOINTS (HARD GATES)

Copilot CANNOT proceed past checkpoint until ALL requirements met:

```yaml
CP0_Foundation:
  - [ ] Workspace structure created
  - [ ] Turborepo + pnpm configured
  - [ ] Docker compose ready
  
CP1_Domain_Complete:
  - [ ] All aggregates implemented
  - [ ] All value objects implemented
  - [ ] All domain events defined
  - [ ] 100% test coverage
  - [ ] No tech dependencies
  
CP2_Application_Layer:
  - [ ] All command handlers
  - [ ] All query handlers
  - [ ] Event publishing works
  - [ ] 95% test coverage
  
CP3_Infrastructure:
  - [ ] Event bus operational
  - [ ] Postgres repositories
  - [ ] DragonflyDB cache
  - [ ] Circuit breakers
  
CP4_Adapters:
  - [ ] RSI Auth adapter
  - [ ] Game.log parser
  - [ ] FleetYards integration
  - [ ] Discord adapter
  - [ ] 85% test coverage
  
CP5_Desktop_UI:
  - [ ] Tauri shell configured
  - [ ] SolidJS + shadcn/ui setup
  - [ ] All screens implemented
  - [ ] Event streaming works
  - [ ] 75% test coverage
  
CP6_E2E_Testing:
  - [ ] All critical flows tested
  - [ ] Performance budgets met
  - [ ] Security audit passed
  
CP7_Production_Ready:
  - [ ] Installer (WiX)
  - [ ] Auto-update configured
  - [ ] Monitoring enabled
  - [ ] Documentation complete
```

---

## 1️⃣1️⃣ DEFINITION OF DONE

A task is DONE only when ALL conditions met:

```yaml
Code:
  - [ ] Follows layer separation
  - [ ] No business logic in adapters
  - [ ] Events used for state changes
  - [ ] Error handling complete
  - [ ] No unwrap() in production
  - [ ] Logging structured

Testing:
  - [ ] Unit tests (domain ≥100%)
  - [ ] Integration tests present
  - [ ] E2E tests for critical flows
  - [ ] Coverage ≥85%
  - [ ] Performance tests pass

Security:
  - [ ] Input validation
  - [ ] Authorization checks
  - [ ] No secrets in code
  - [ ] cargo-audit passed
  - [ ] ToS compliance verified

Documentation:
  - [ ] Public APIs documented
  - [ ] Examples provided
  - [ ] ADR created (if architectural)
  - [ ] README updated

Performance:
  - [ ] Meets performance budget
  - [ ] No N+1 queries
  - [ ] Caching applied
  - [ ] Memory leaks checked

Accessibility (UI):
  - [ ] Keyboard navigation
  - [ ] Screen reader compatible
  - [ ] Color contrast ≥4.5:1
  - [ ] No A11y violations
```

---

## 1️⃣2️⃣ COPILOT FINAL INSTRUCTIONS

```
YOU ARE GITHUB COPILOT IMPLEMENTING SC MANAGER V5.

ABSOLUTE RULES:
1. Read this ENTIRE document before ANY code generation
2. Follow EVERY rule without interpretation
3. Use EXACT tech stack (no substitutes)
4. Respect layer boundaries (ZERO violations)
5. STOP if uncertain (don't guess)
6. Never skip tests
7. Never use forbidden technologies
8. Always check Definition of Done

BEFORE EVERY CODE GENERATION:
✓ Run self-check protocol (Section 0)
✓ Verify layer separation
✓ Check tech stack compliance
✓ Ensure tests included
✓ Verify error handling
✓ Check performance impact
✓ Confirm ToS compliance

IF ANY CHECK FAILS:
→ STOP
→ Request clarification
→ Do NOT proceed

REMEMBER:
- Domain = Pure business logic (NO tech)
- Application = Orchestration only
- Adapters = Dumb translators
- Events = ALL state changes
- Tests = MANDATORY
- Performance = BUDGETED
- Security = MANDATORY
- ToS = LAW

THIS DOCUMENT IS:
✓ Complete
✓ Unambiguous
✓ Non-negotiable
✓ Your single source of truth

DEVIATIONS = REJECTED

NOW PROCEED WITH CONFIDENCE.
```

---

**STATUS:** FINAL & ABSOLUTE  
**VERSION:** 5.0.0  
**AUTHORITY:** MAXIMUM  
**UPDATES:** Only with explicit approval  
**QUESTIONS:** Stop and ask, never assume

