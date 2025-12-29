---
title: COPILOT_MASTER_INSTRUCTION_V6_COMPLETE_WITH_PLUGINS
version: 6.0.0
date: 2025-12-27
status: ABSOLUTE - ZERO DEVIATION ALLOWED
priority: MAXIMUM
binding: NON-NEGOTIABLE
features: BASE + PLUGIN_SDK + GRINDING_PLUGIN + ROLEPLAY_PLUGIN + DESKTOP_INSTALLER
---

# 🎯 SC MANAGER V6 - COMPLETE WITH PLUGIN SYSTEM

**PLUGIN SDK | GRINDING PLUGIN | ROLEPLAY PLUGIN | WINDOWS INSTALLER | ZERO TOLERANCE**

---

## 📚 TABLE OF CONTENTS

0. [Meta Rules](#0-meta-rules)
1. [Tech Stack](#1-tech-stack)
2. [Architecture](#2-architecture)
3. [Project Structure](#3-project-structure)
4. [Core Domain Model](#4-core-domain-model)
5. [Plugin SDK](#5-plugin-sdk-complete)
6. [Grinding Plugin](#6-grinding-plugin-complete)
7. [Roleplay Plugin](#7-roleplay-plugin-template)
8. [Windows Installer](#8-windows-installer-idc-10)
9. [Implementation Standards](#9-implementation-standards)
10. [Testing](#10-testing)
11. [Performance & Security](#11-performance--security)
12. [Checkpoints](#12-checkpoints)
13. [Definition of Done](#13-definition-of-done)

---

## 0️⃣ META RULES

### Rule 0.1: Absolute Hierarchy
```
1. THIS DOCUMENT = LAW
2. Plugin SDK Rules (for plugins)
3. IDC-10 Guidelines (for installer/UI)
4. Star Citizen ToS (CRITICAL)
5. Best Practices
```

### Rule 0.2: Self-Check Protocol (MANDATORY)
```
Copilot MUST verify BEFORE generating ANY code:
✓ Read relevant section of this document?
✓ Layer separation correct?
✓ Using exact tech stack?
✓ No business logic in adapters?
✓ Events for state changes?
✓ Tests included?
✓ Error handling present?
✓ No unwrap() in production?
✓ Performance budget respected?
✓ ToS compliance verified?
✓ Plugin isolation maintained? (if plugin)
✓ No gameplay automation? (GRINDING)
✓ Windows-native behavior? (INSTALLER)

IF ANY ❌ → STOP
```

### Rule 0.3: Stop Conditions
- Requirement contradicts document
- No clear implementation path
- ToS violation (gameplay automation)
- Layer separation break
- Performance budget exceed
- Plugin tries to access Core directly
- Installer uses non-native patterns

### Rule 0.4: Forbidden Actions
```yaml
NEVER:
  - Skip tests
  - Merge layers
  - Use forbidden tech
  - Business logic in adapters
  - Plugins accessing Core directly
  - Plugins issuing Commands
  - Gameplay automation
  - Auto-start without consent
```

---

## 1️⃣ TECH STACK

### Backend
```yaml
Language: Rust 1.75+
Framework: Axum 0.7+
Database: PostgreSQL 16+
Cache: DragonflyDB 1.13+
Event_Bus: In-Memory (desktop) / NATS (enterprise)
```

### Desktop
```yaml
Shell: Tauri 2.0+
Frontend: SolidJS 1.8+ (NOT React)
UI: shadcn/ui + Radix (NOT Fluent UI)
Styling: Tailwind CSS
State: SolidJS Stores
TypeScript: 5.3+ (strict)
```

### Plugin System (NEW)
```yaml
Runtime: Isolated sandbox
Language: TypeScript/JavaScript
API: Read-only event subscription
Storage: Scoped (per-plugin)
UI: Optional (SolidJS components)
```

### Build & Tools
```yaml
Monorepo: Turborepo
Package_Manager: pnpm 8.0+
Installer: WiX Toolset v4
Linter: biome (NOT eslint)
```

### ⛔ FORBIDDEN
```
Electron, React, Fluent UI, npm, eslint, Redis, MongoDB
```

---

## 2️⃣ ARCHITECTURE

### Core Architecture
```
┌──────────────────────────────┐
│ UI (SolidJS + Plugins)      │
├──────────────────────────────┤
│ Application Layer            │
├──────────────────────────────┤
│ Domain Layer (PURE)          │
├──────────────────────────────┤
│ Event Bus                    │
├──────────────────────────────┤
│ Adapters | Read Models       │
└──────────────────────────────┘
```

### Plugin Architecture (NEW)
```
┌─────────────────────────────┐
│ Plugin UI (Optional)        │
├─────────────────────────────┤
│ Plugin Logic                │ ← Isolated
├─────────────────────────────┤
│ Plugin Context (Read-Only)  │
│ - Event Subscription        │
│ - Read Model Access         │
│ - Scoped Storage            │
└─────────────────────────────┘
         ↑ Events Only
┌─────────────────────────────┐
│ Core Event Bus              │
└─────────────────────────────┘
```

**Critical Rules:**
- Plugins CANNOT access Core directly
- Plugins CANNOT issue Commands
- Plugins ONLY subscribe to Events
- Plugins have scoped storage
- Plugins are fully isolatable

---

## 3️⃣ PROJECT STRUCTURE

```
sc-manager/
├── apps/
│   └── desktop/
│       ├── src/                    # SolidJS
│       │   ├── components/
│       │   ├── pages/
│       │   ├── stores/
│       │   └── plugins/            # NEW: Plugin loader
│       │       ├── loader.ts
│       │       ├── sandbox.ts
│       │       └── registry.ts
│       └── src-tauri/              # Rust
│           ├── src/
│           └── wix/                # NEW: Installer
│
├── services/
│   ├── core-domain/                # PURE
│   │   └── src/
│   │       ├── organization/
│   │       ├── operation/
│   │       ├── member/
│   │       ├── fleet/
│   │       └── diplomacy/
│   ├── core-application/
│   └── gateway/
│
├── adapters/
│   ├── adapter-rsi-auth/
│   ├── adapter-gamelog/            # Read-only
│   ├── adapter-discord/
│   └── ...
│
├── infrastructure/
│   ├── eventbus/
│   ├── persistence/
│   ├── cache/
│   ├── resilience/
│   └── plugin-sdk/                 # NEW: Plugin SDK
│       └── src/
│           ├── runtime.rs
│           ├── sandbox.rs
│           ├── permissions.rs
│           └── storage.rs
│
├── plugins/                        # NEW: Plugin directory
│   ├── grinding/                   # Grinding Plugin
│   │   ├── plugin.json
│   │   ├── index.ts
│   │   ├── domain/
│   │   │   ├── mission.ts
│   │   │   ├── grinding_goal.ts
│   │   │   └── mission_progress.ts
│   │   ├── ui/
│   │   │   ├── GrindingDashboard.tsx
│   │   │   ├── MissionList.tsx
│   │   │   └── GoalProgress.tsx
│   │   ├── storage/
│   │   │   └── schema.ts
│   │   └── tests/
│   │
│   ├── roleplay/                   # Roleplay Plugin
│   │   ├── plugin.json
│   │   ├── index.ts
│   │   ├── domain/
│   │   │   ├── character.ts
│   │   │   ├── rp_event.ts
│   │   │   └── lore.ts
│   │   ├── ui/
│   │   │   ├── CharacterSheet.tsx
│   │   │   └── Timeline.tsx
│   │   └── tests/
│   │
│   └── _template/                  # Plugin Template
│       ├── plugin.json
│       ├── index.ts
│       ├── domain/
│       ├── ui/
│       └── tests/
│
├── installer/                      # NEW: Windows Installer
│   ├── wix/
│   │   ├── main.wxs
│   │   ├── ui.wxs
│   │   └── features.wxs
│   ├── assets/
│   │   └── icons/
│   └── scripts/
│
├── docs/
│   ├── architecture/
│   │   ├── ADR-010-plugin-sdk.md
│   │   ├── ADR-011-grinding-plugin.md
│   │   └── ADR-012-windows-installer.md
│   └── plugins/
│       ├── PLUGIN_DEVELOPMENT_GUIDE.md
│       └── API_REFERENCE.md
│
├── turbo.json
├── pnpm-workspace.yaml
└── Cargo.toml
```

---

## 4️⃣ CORE DOMAIN MODEL (BASE)

### Organization Aggregate
```rust
pub struct Organization {
    id: OrganizationId,
    name: OrganizationName,
    tag: OrganizationTag,
    members: Vec<MemberId>,
    // ...
}
```

### Operation Aggregate
```rust
pub struct Operation {
    id: OperationId,
    name: OperationName,
    operation_type: OperationType,
    status: OperationStatus,
    participants: Vec<Participant>,
    // ...
}
```

### Member Aggregate
```rust
pub struct Member {
    id: MemberId,
    rsi_identity: RsiIdentity,
    roles: Vec<Role>,
    qualifications: Vec<Qualification>,
    // ...
}
```

### Fleet Aggregate
```rust
pub struct Fleet {
    id: FleetId,
    name: FleetName,
    ships: Vec<Ship>,
    readiness_state: ReadinessState,
}
```

### Diplomacy Aggregate
```rust
pub struct DiplomaticRelation {
    id: DiplomaticRelationId,
    organization_a: OrganizationId,
    organization_b: OrganizationId,
    status: DiplomaticStatus,
}
```

**Note:** Core domain contains NO grinding or roleplay logic.
These are implemented as PLUGINS.

---

## 5️⃣ PLUGIN SDK (COMPLETE)

### 5.1 Plugin Philosophy

**Plugins are:**
- Optional add-ons
- Fully isolated
- Event-driven
- Read-only to Core
- Disableable without side effects

**Plugins are NOT:**
- Mods
- Cheats
- Core extensions
- Gameplay automation

### 5.2 Plugin Lifecycle

```typescript
interface Plugin {
  // Called once when plugin loads
  onLoad(ctx: PluginContext): void;
  
  // Called when plugin is enabled
  onEnable(): void;
  
  // Called for every event
  onEvent(event: PluginEvent): void;
  
  // Called when plugin is disabled
  onDisable(): void;
  
  // Metadata
  metadata(): PluginMetadata;
}
```

### 5.3 Plugin Interface (TypeScript)

```typescript
// infrastructure/plugin-sdk/types.ts

interface PluginMetadata {
  id: string;                    // Unique identifier (e.g. "grinding")
  name: string;                  // Display name
  version: string;               // SemVer (e.g. "1.0.0")
  engine: string;                // Required engine version (e.g. ">=1.0.0")
  author: string;                // Plugin author
  description: string;           // Brief description
  permissions: Permission[];     // Required capabilities
  ui: boolean;                   // Has UI components?
}

type Permission = 
  | "read-events"                // Subscribe to events
  | "read-data"                  // Read Core data
  | "storage-local"              // Local storage access
  | "ui-render";                 // Render UI

interface PluginContext {
  // Event subscription (read-only)
  eventStream: ReadonlyEventStream;
  
  // Data access (read-only)
  readApi: ReadModelApi;
  
  // Plugin-scoped storage
  storage: PluginStorage;
  
  // Logging
  logger: PluginLogger;
}

interface ReadonlyEventStream {
  subscribe<T extends DomainEvent>(
    eventType: string,
    handler: (event: T) => void | Promise<void>
  ): Subscription;
}

interface ReadModelApi {
  // Query read models only
  query<T>(queryName: string, params?: any): Promise<T>;
}

interface PluginStorage {
  get<T>(key: string): Promise<T | null>;
  set<T>(key: string, value: T): Promise<void>;
  delete(key: string): Promise<void>;
  list(prefix?: string): Promise<string[]>;
}

interface PluginLogger {
  info(message: string, meta?: any): void;
  warn(message: string, meta?: any): void;
  error(message: string, meta?: any): void;
  debug(message: string, meta?: any): void;
}
```

### 5.4 Plugin Manifest (plugin.json)

```json
{
  "id": "grinding",
  "name": "Mission Grinding Tracker",
  "version": "1.0.0",
  "engine": ">=1.0.0",
  "author": "SC Manager Team",
  "description": "Track mission grinding progress and goals",
  "permissions": [
    "read-events",
    "read-data",
    "storage-local",
    "ui-render"
  ],
  "ui": true,
  "main": "index.ts",
  "routes": [
    {
      "path": "/grinding",
      "component": "GrindingDashboard"
    }
  ]
}
```

### 5.5 Plugin Security (CRITICAL)

**Sandbox Enforcement:**
```rust
// infrastructure/plugin-sdk/src/sandbox.rs

pub struct PluginSandbox {
    plugin_id: String,
    permissions: HashSet<Permission>,
    memory_limit: usize,           // 50MB default
    cpu_time_limit: Duration,      // 1s default
}

impl PluginSandbox {
    pub fn check_permission(&self, perm: Permission) -> Result<()> {
        if !self.permissions.contains(&perm) {
            return Err(PluginError::InsufficientPermissions);
        }
        Ok(())
    }
    
    pub async fn execute<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> T + Send + 'static,
    {
        // Enforce resource limits
        self.check_memory_limit()?;
        
        // Execute with timeout
        tokio::time::timeout(
            self.cpu_time_limit,
            tokio::task::spawn_blocking(f)
        ).await??
    }
}
```

**Permission Model:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    ReadEvents,      // Subscribe to domain events
    ReadData,        // Query read models
    StorageLocal,    // Access plugin-scoped storage
    UiRender,        // Render UI components
}

// ❌ FORBIDDEN Permissions (never granted):
// - WriteCommands (cannot issue commands)
// - AccessCore (cannot access Core directly)
// - FileSystem (cannot access file system)
// - Network (cannot make network requests)
```

### 5.6 Plugin Registry

```typescript
// apps/desktop/src/plugins/registry.ts

class PluginRegistry {
  private plugins: Map<string, LoadedPlugin> = new Map();
  
  async loadPlugin(manifest: PluginMetadata): Promise<void> {
    // 1. Validate manifest
    this.validateManifest(manifest);
    
    // 2. Check permissions
    this.checkPermissions(manifest.permissions);
    
    // 3. Create sandbox
    const sandbox = new PluginSandbox(manifest);
    
    // 4. Load plugin code
    const plugin = await this.loadPluginCode(manifest.main);
    
    // 5. Create context
    const context = this.createContext(manifest.id, manifest.permissions);
    
    // 6. Initialize plugin
    await sandbox.execute(() => plugin.onLoad(context));
    
    // 7. Register
    this.plugins.set(manifest.id, { manifest, plugin, sandbox, context });
  }
  
  async enablePlugin(pluginId: string): Promise<void> {
    const loaded = this.plugins.get(pluginId);
    if (!loaded) throw new Error("Plugin not loaded");
    
    await loaded.sandbox.execute(() => loaded.plugin.onEnable());
    loaded.enabled = true;
  }
  
  async disablePlugin(pluginId: string): Promise<void> {
    const loaded = this.plugins.get(pluginId);
    if (!loaded) return;
    
    await loaded.sandbox.execute(() => loaded.plugin.onDisable());
    loaded.enabled = false;
  }
  
  async unloadPlugin(pluginId: string): Promise<void> {
    await this.disablePlugin(pluginId);
    this.plugins.delete(pluginId);
  }
}
```

### 5.7 Event Distribution to Plugins

```typescript
// infrastructure/eventbus/plugin_distributor.ts

class PluginEventDistributor {
  constructor(
    private registry: PluginRegistry,
    private eventBus: EventBus
  ) {}
  
  async start(): void {
    // Subscribe to all domain events
    this.eventBus.subscribe("*", async (event) => {
      await this.distributeToPlugins(event);
    });
  }
  
  private async distributeToPlugins(event: DomainEvent): Promise<void> {
    const plugins = this.registry.getEnabledPlugins();
    
    // Distribute to each plugin in parallel
    await Promise.allSettled(
      plugins.map(async (loaded) => {
        // Check if plugin has read-events permission
        if (!loaded.context.hasPermission(Permission.ReadEvents)) {
          return;
        }
        
        try {
          // Execute in sandbox with timeout
          await loaded.sandbox.execute(() => 
            loaded.plugin.onEvent(event)
          );
        } catch (err) {
          loaded.context.logger.error("Event handling failed", { err, event });
        }
      })
    );
  }
}
```

### 5.8 Plugin Template

```typescript
// plugins/_template/index.ts

export default class TemplatePlugin implements Plugin {
  private ctx!: PluginContext;
  
  metadata(): PluginMetadata {
    return {
      id: "template-plugin",
      name: "Template Plugin",
      version: "1.0.0",
      engine: ">=1.0.0",
      author: "Your Name",
      description: "A template plugin",
      permissions: ["read-events", "storage-local"],
      ui: false
    };
  }
  
  onLoad(ctx: PluginContext): void {
    this.ctx = ctx;
    ctx.logger.info("Template plugin loaded");
    
    // Subscribe to events
    ctx.eventStream.subscribe("OperationPlanned", this.handleOperationPlanned);
  }
  
  onEnable(): void {
    this.ctx.logger.info("Template plugin enabled");
  }
  
  async onEvent(event: PluginEvent): Promise<void> {
    // Handle events
    switch (event.event_type) {
      case "MemberAdded":
        await this.handleMemberAdded(event);
        break;
      // ...
    }
  }
  
  onDisable(): void {
    this.ctx.logger.info("Template plugin disabled");
  }
  
  private handleOperationPlanned = async (event: OperationPlanned) => {
    this.ctx.logger.info("Operation planned", { id: event.operation_id });
    // React to event...
  };
  
  private async handleMemberAdded(event: MemberAdded): Promise<void> {
    // Store something
    await this.ctx.storage.set(`member:${event.member_id}`, {
      joined_at: event.timestamp
    });
  }
}
```

---

## 6️⃣ GRINDING PLUGIN (COMPLETE)

### 6.1 Grinding Philosophy

**ToS-SAFE Grinding Tracking:**
- Manual reporting ONLY
- Officer verification required
- Read-only Game.log parsing (future) — Planned: add an **adapter** (`adapter-gamelog`) that performs **local, read-only** parsing of the game's `Game.log` to surface *suggested* mission completions for *manual* verification. Requirements: user opt-in, parsing is local only (no network), parser emits **suggestions** only (no automatic verification or state changes), and all outputs require officer verification before affecting plugin state. See ADR-013 (Design & Constraints) for details.
- NO automation
- NO auto-completion
- Pure tracking & coordination

### 6.2 Grinding Plugin Manifest

```json
{
  "id": "grinding",
  "name": "Mission Grinding Tracker",
  "version": "1.0.0",
  "engine": ">=1.0.0",
  "author": "SC Manager Team",
  "description": "Track mission grinding progress, goals, and reputation (ToS-safe, no automation)",
  "permissions": [
    "read-events",
    "read-data",
    "storage-local",
    "ui-render"
  ],
  "ui": true,
  "main": "index.ts",
  "routes": [
    {
      "path": "/grinding",
      "component": "GrindingDashboard"
    }
  ],
  "tos_compliance": {
    "no_automation": true,
    "manual_reporting_only": true,
    "officer_verification_required": true
  }
}
```

### 6.3 Grinding Domain (Plugin-Internal)

```typescript
// plugins/grinding/domain/mission.ts

export interface Mission {
  id: string;                          // UUID
  name: string;                        // e.g. "Wikelo Delivery"
  category: MissionCategory;
  npc_or_faction: string;              // e.g. "Wikelo"
  difficulty: 1 | 2 | 3 | 4 | 5;
  reputation_gain: number;             // Estimated
  is_repeatable: boolean;
  metadata: {
    location?: string;
    recommended_ships?: string[];
    notes?: string;
  };
}

export type MissionCategory = 
  | "combat" 
  | "trade" 
  | "support" 
  | "training" 
  | "mining" 
  | "exploration";

export class MissionEntity {
  constructor(private mission: Mission) {}
  
  // Business Rules
  static create(data: Omit<Mission, "id">): Result<Mission, ValidationError> {
    // Validate name (3-200 chars)
    if (data.name.length < 3 || data.name.length > 200) {
      return Err({ field: "name", message: "Must be 3-200 characters" });
    }
    
    // Validate difficulty
    if (data.difficulty < 1 || data.difficulty > 5) {
      return Err({ field: "difficulty", message: "Must be 1-5" });
    }
    
    // Validate reputation gain
    if (data.reputation_gain <= 0) {
      return Err({ field: "reputation_gain", message: "Must be positive" });
    }
    
    return Ok({
      ...data,
      id: crypto.randomUUID()
    });
  }
}
```

```typescript
// plugins/grinding/domain/grinding_goal.ts

export interface GrindingGoal {
  id: string;
  mission_id: string;
  target_completions: number;
  scope: "personal" | "squad" | "organization";
  owner_id: string;                    // Member ID or Org ID
  start_date: Date;
  end_date?: Date;
  status: "active" | "completed" | "expired" | "cancelled";
  current_completions: number;
}

export class GrindingGoalEntity {
  constructor(private goal: GrindingGoal) {}
  
  // Business Rules
  static create(
    data: Omit<GrindingGoal, "id" | "current_completions" | "status">
  ): Result<GrindingGoal, ValidationError> {
    // Target must be positive
    if (data.target_completions <= 0) {
      return Err({ field: "target_completions", message: "Must be positive" });
    }
    
    // Scope limits
    if (data.scope === "personal" && data.target_completions > 100) {
      return Err({ field: "target_completions", message: "Personal goals limited to 100" });
    }
    
    if (data.scope === "squad" && data.target_completions > 500) {
      return Err({ field: "target_completions", message: "Squad goals limited to 500" });
    }
    
    // End date validation
    if (data.end_date && data.end_date <= data.start_date) {
      return Err({ field: "end_date", message: "Must be after start date" });
    }
    
    return Ok({
      ...data,
      id: crypto.randomUUID(),
      current_completions: 0,
      status: "active"
    });
  }
  
  recordProgress(completions: number): Result<GrindingGoal, DomainError> {
    // Check status
    if (this.goal.status !== "active") {
      return Err({ type: "GoalNotActive", message: "Cannot add progress to inactive goal" });
    }
    
    // Check expiry
    if (this.goal.end_date && new Date() > this.goal.end_date) {
      this.goal.status = "expired";
      return Err({ type: "GoalExpired", message: "Goal has expired" });
    }
    
    // Update progress
    this.goal.current_completions += completions;
    
    // Auto-complete
    if (this.goal.current_completions >= this.goal.target_completions) {
      this.goal.status = "completed";
    }
    
    return Ok(this.goal);
  }
}
```

```typescript
// plugins/grinding/domain/mission_progress.ts

export interface MissionProgress {
  id: string;
  mission_id: string;
  member_id: string;
  completions: number;
  last_completed_at: Date;
  verification_state: "pending" | "verified" | "rejected";
  verification_method: "manual" | "log" | "officer";
  verified_by?: string;                // Officer ID
  verified_at?: Date;
}

export class MissionProgressEntity {
  constructor(private progress: MissionProgress) {}
  
  // ToS-SAFE: Manual reporting only
  static reportCompletion(
    mission_id: string,
    member_id: string
  ): MissionProgress {
    return {
      id: crypto.randomUUID(),
      mission_id,
      member_id,
      completions: 1,
      last_completed_at: new Date(),
      verification_state: "pending",     // ✅ ALWAYS pending initially
      verification_method: "manual"
    };
  }
  
  // ToS-SAFE: Officer verification required
  verify(officer_id: string): Result<MissionProgress, DomainError> {
    if (this.progress.verification_state !== "pending") {
      return Err({ type: "AlreadyVerified" });
    }
    
    this.progress.verification_state = "verified";
    this.progress.verified_by = officer_id;
    this.progress.verified_at = new Date();
    
    return Ok(this.progress);
  }
  
  reject(officer_id: string, reason: string): Result<MissionProgress, DomainError> {
    if (this.progress.verification_state !== "pending") {
      return Err({ type: "AlreadyProcessed" });
    }
    
    this.progress.verification_state = "rejected";
    this.progress.verified_by = officer_id;
    this.progress.verified_at = new Date();
    
    return Ok(this.progress);
  }
}
```

### 6.4 Grinding Plugin Implementation

```typescript
// plugins/grinding/index.ts

export default class GrindingPlugin implements Plugin {
  private ctx!: PluginContext;
  private missions: Map<string, Mission> = new Map();
  private goals: Map<string, GrindingGoal> = new Map();
  private progress: Map<string, MissionProgress> = new Map();
  
  metadata(): PluginMetadata {
    return {
      id: "grinding",
      name: "Mission Grinding Tracker",
      version: "1.0.0",
      engine: ">=1.0.0",
      author: "SC Manager Team",
      description: "Track mission grinding (ToS-safe, no automation)",
      permissions: ["read-events", "read-data", "storage-local", "ui-render"],
      ui: true
    };
  }
  
  async onLoad(ctx: PluginContext): Promise<void> {
    this.ctx = ctx;
    ctx.logger.info("Grinding plugin loaded");
    
    // Load persisted data
    await this.loadPersistedData();
    
    // Subscribe to relevant events (read-only)
    ctx.eventStream.subscribe("MemberAdded", this.handleMemberAdded);
    ctx.eventStream.subscribe("OperationCompleted", this.handleOperationCompleted);
  }
  
  onEnable(): void {
    this.ctx.logger.info("Grinding plugin enabled");
  }
  
  async onEvent(event: PluginEvent): Promise<void> {
    // Events are handled via subscriptions
  }
  
  onDisable(): void {
    this.ctx.logger.info("Grinding plugin disabled");
  }
  
  // Plugin-specific methods (called from UI)
  async createMission(data: Omit<Mission, "id">): Promise<Result<Mission, ValidationError>> {
    const result = MissionEntity.create(data);
    
    if (result.ok) {
      const mission = result.value;
      this.missions.set(mission.id, mission);
      await this.ctx.storage.set(`mission:${mission.id}`, mission);
      this.ctx.logger.info("Mission created", { id: mission.id });
    }
    
    return result;
  }
  
  async createGrindingGoal(
    data: Omit<GrindingGoal, "id" | "current_completions" | "status">
  ): Promise<Result<GrindingGoal, ValidationError>> {
    const result = GrindingGoalEntity.create(data);
    
    if (result.ok) {
      const goal = result.value;
      this.goals.set(goal.id, goal);
      await this.ctx.storage.set(`goal:${goal.id}`, goal);
      this.ctx.logger.info("Grinding goal created", { id: goal.id });
    }
    
    return result;
  }
  
  // ToS-SAFE: Manual reporting only
  async reportMissionCompletion(
    mission_id: string,
    member_id: string
  ): Promise<MissionProgress> {
    const progress = MissionProgressEntity.reportCompletion(mission_id, member_id);
    
    this.progress.set(progress.id, progress);
    await this.ctx.storage.set(`progress:${progress.id}`, progress);
    
    this.ctx.logger.info("Mission completion reported (pending verification)", {
      mission_id,
      member_id,
      progress_id: progress.id
    });
    
    return progress;
  }
  
  // ToS-SAFE: Officer verification
  async verifyMissionCompletion(
    progress_id: string,
    officer_id: string
  ): Promise<Result<MissionProgress, DomainError>> {
    const progress = this.progress.get(progress_id);
    if (!progress) {
      return Err({ type: "NotFound", message: "Progress not found" });
    }
    
    const entity = new MissionProgressEntity(progress);
    const result = entity.verify(officer_id);
    
    if (result.ok) {
      const verified = result.value;
      this.progress.set(progress_id, verified);
      await this.ctx.storage.set(`progress:${progress_id}`, verified);
      
      // Update goal progress
      const goal = Array.from(this.goals.values()).find(
        g => g.mission_id === verified.mission_id && g.owner_id === verified.member_id
      );
      
      if (goal) {
        const goalEntity = new GrindingGoalEntity(goal);
        const updated = goalEntity.recordProgress(1);
        
        if (updated.ok) {
          this.goals.set(goal.id, updated.value);
          await this.ctx.storage.set(`goal:${goal.id}`, updated.value);
        }
      }
      
      this.ctx.logger.info("Mission completion verified", {
        progress_id,
        officer_id
      });
    }
    
    return result;
  }
  
  // Query methods for UI
  async getMissions(): Promise<Mission[]> {
    return Array.from(this.missions.values());
  }
  
  async getGrindingGoals(owner_id: string): Promise<GrindingGoal[]> {
    return Array.from(this.goals.values())
      .filter(g => g.owner_id === owner_id);
  }
  
  async getPendingVerifications(): Promise<MissionProgress[]> {
    return Array.from(this.progress.values())
      .filter(p => p.verification_state === "pending");
  }
  
  async getGrindingStatus(entity_id: string): Promise<GrindingStatus> {
    const goals = this.getGrindingGoals(entity_id);
    const progress = Array.from(this.progress.values())
      .filter(p => p.member_id === entity_id && p.verification_state === "verified");
    
    return {
      entity_id,
      active_goals: goals.filter(g => g.status === "active").length,
      completed_goals: goals.filter(g => g.status === "completed").length,
      total_completions: progress.reduce((sum, p) => sum + p.completions, 0),
      reputation_level: this.calculateReputationLevel(progress)
    };
  }
  
  // Private methods
  private async loadPersistedData(): Promise<void> {
    const missionKeys = await this.ctx.storage.list("mission:");
    for (const key of missionKeys) {
      const mission = await this.ctx.storage.get<Mission>(key);
      if (mission) {
        this.missions.set(mission.id, mission);
      }
    }
    
    const goalKeys = await this.ctx.storage.list("goal:");
    for (const key of goalKeys) {
      const goal = await this.ctx.storage.get<GrindingGoal>(key);
      if (goal) {
        this.goals.set(goal.id, goal);
      }
    }
    
    const progressKeys = await this.ctx.storage.list("progress:");
    for (const key of progressKeys) {
      const progress = await this.ctx.storage.get<MissionProgress>(key);
      if (progress) {
        this.progress.set(progress.id, progress);
      }
    }
  }
  
  private handleMemberAdded = async (event: MemberAdded) => {
    this.ctx.logger.info("New member added (grinding plugin notified)", {
      member_id: event.member_id
    });
    // Could initialize default grinding goals here
  };
  
  private handleOperationCompleted = async (event: OperationCompleted) => {
    this.ctx.logger.info("Operation completed (grinding plugin notified)", {
      operation_id: event.operation_id
    });
    // Could suggest mission grinding opportunities based on operation type
  };
  
  private calculateReputationLevel(progress: MissionProgress[]): number {
    // Simple calculation: sum of all verified completions
    // In reality, would weight by mission difficulty, faction, etc.
    return progress.reduce((sum, p) => sum + p.completions, 0);
  }
}

// Export for registration
export { GrindingPlugin };
```

### 6.5 Grinding UI Components

```tsx
// plugins/grinding/ui/GrindingDashboard.tsx

import { createSignal, createEffect, For } from "solid-js";
import { Card } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";

export function GrindingDashboard() {
  const [goals, setGoals] = createSignal<GrindingGoal[]>([]);
  const [missions, setMissions] = createSignal<Mission[]>([]);
  const [status, setStatus] = createSignal<GrindingStatus | null>(null);
  
  createEffect(async () => {
    // Load data from plugin
    const plugin = window.pluginRegistry.get("grinding");
    if (plugin) {
      const userGoals = await plugin.getGrindingGoals(currentUserId);
      const allMissions = await plugin.getMissions();
      const userStatus = await plugin.getGrindingStatus(currentUserId);
      
      setGoals(userGoals);
      setMissions(allMissions);
      setStatus(userStatus);
    }
  });
  
  const handleReportCompletion = async (missionId: string) => {
    const plugin = window.pluginRegistry.get("grinding");
    if (plugin) {
      await plugin.reportMissionCompletion(missionId, currentUserId);
      // Refresh data
    }
  };
  
  return (
    <div class="p-6 space-y-6">
      <div class="flex justify-between items-center">
        <h1 class="text-3xl font-bold">Mission Grinding</h1>
        <Button onClick={() => showCreateGoalDialog()}>
          New Goal
        </Button>
      </div>
      
      {/* Status Overview */}
      <Card class="p-6">
        <h2 class="text-xl font-semibold mb-4">Your Status</h2>
        <div class="grid grid-cols-3 gap-4">
          <div>
            <div class="text-sm text-muted-foreground">Active Goals</div>
            <div class="text-2xl font-bold">{status()?.active_goals ?? 0}</div>
          </div>
          <div>
            <div class="text-sm text-muted-foreground">Completed Goals</div>
            <div class="text-2xl font-bold">{status()?.completed_goals ?? 0}</div>
          </div>
          <div>
            <div class="text-sm text-muted-foreground">Total Completions</div>
            <div class="text-2xl font-bold">{status()?.total_completions ?? 0}</div>
          </div>
        </div>
      </Card>
      
      {/* Active Goals */}
      <div>
        <h2 class="text-xl font-semibold mb-4">Active Goals</h2>
        <div class="space-y-4">
          <For each={goals().filter(g => g.status === "active")}>
            {(goal) => (
              <Card class="p-4">
                <div class="flex justify-between items-start mb-2">
                  <div>
                    <h3 class="font-semibold">
                      {missions().find(m => m.id === goal.mission_id)?.name}
                    </h3>
                    <p class="text-sm text-muted-foreground">
                      {goal.scope} goal • {goal.current_completions} / {goal.target_completions}
                    </p>
                  </div>
                  <Button 
                    size="sm"
                    onClick={() => handleReportCompletion(goal.mission_id)}
                  >
                    Report Completion
                  </Button>
                </div>
                <Progress 
                  value={(goal.current_completions / goal.target_completions) * 100}
                  class="mt-2"
                />
              </Card>
            )}
          </For>
        </div>
      </div>
      
      {/* Available Missions */}
      <div>
        <h2 class="text-xl font-semibold mb-4">Available Missions</h2>
        <div class="grid grid-cols-2 gap-4">
          <For each={missions().filter(m => m.is_repeatable)}>
            {(mission) => (
              <Card class="p-4">
                <h3 class="font-semibold mb-2">{mission.name}</h3>
                <div class="text-sm space-y-1">
                  <div>Category: {mission.category}</div>
                  <div>Faction: {mission.npc_or_faction}</div>
                  <div>Difficulty: {mission.difficulty}/5</div>
                  <div>Rep Gain: +{mission.reputation_gain}</div>
                </div>
                <Button 
                  class="mt-4 w-full" 
                  variant="outline"
                  onClick={() => showCreateGoalForMission(mission.id)}
                >
                  Create Goal
                </Button>
              </Card>
            )}
          </For>
        </div>
      </div>
    </div>
  );
}
```

```tsx
// plugins/grinding/ui/VerificationQueue.tsx (Officer View)

export function VerificationQueue() {
  const [pending, setPending] = createSignal<MissionProgress[]>([]);
  
  createEffect(async () => {
    const plugin = window.pluginRegistry.get("grinding");
    if (plugin) {
      const pendingVerifications = await plugin.getPendingVerifications();
      setPending(pendingVerifications);
    }
  });
  
  const handleVerify = async (progressId: string) => {
    const plugin = window.pluginRegistry.get("grinding");
    if (plugin) {
      await plugin.verifyMissionCompletion(progressId, currentOfficerId);
      // Refresh
    }
  };
  
  const handleReject = async (progressId: string) => {
    // Implementation
  };
  
  return (
    <Card class="p-6">
      <h2 class="text-xl font-semibold mb-4">Pending Verifications</h2>
      <div class="space-y-2">
        <For each={pending()}>
          {(progress) => (
            <div class="flex justify-between items-center p-3 border rounded">
              <div>
                <div class="font-medium">
                  {/* Mission name */}
                </div>
                <div class="text-sm text-muted-foreground">
                  Reported by {progress.member_id} at {progress.last_completed_at.toLocaleString()}
                </div>
              </div>
              <div class="space-x-2">
                <Button 
                  size="sm" 
                  onClick={() => handleVerify(progress.id)}
                >
                  Verify
                </Button>
                <Button 
                  size="sm" 
                  variant="destructive"
                  onClick={() => handleReject(progress.id)}
                >
                  Reject
                </Button>
              </div>
            </div>
          )}
        </For>
      </div>
    </Card>
  );
}
```

---

## 7️⃣ ROLEPLAY PLUGIN (TEMPLATE)

### 7.1 Roleplay Plugin Manifest

```json
{
  "id": "roleplay",
  "name": "RolePlay & Lore System",
  "version": "1.0.0",
  "engine": ">=1.0.0",
  "author": "SC Manager Team",
  "description": "Manage character backgrounds, RP events, and lore (no gameplay impact)",
  "permissions": [
    "read-events",
    "read-data",
    "storage-local",
    "ui-render"
  ],
  "ui": true,
  "main": "index.ts"
}
```

### 7.2 Roleplay Domain (Simplified)

```typescript
// plugins/roleplay/domain/character.ts

export interface RPCharacter {
  id: string;
  member_id: string;                   // Links to Core Member
  name: string;
  call_sign: string;
  background: string;
  personality_traits: string[];
  skills: RPSkill[];
  relationships: RPRelationship[];
  ships: RPShipPreference[];
  timeline: RPTimelineEntry[];
}

export interface RPSkill {
  name: string;                        // e.g. "Pilot", "Medic"
  level: "novice" | "competent" | "expert" | "master";
  description: string;
}

export interface RPRelationship {
  character_id: string;
  type: "friend" | "rival" | "mentor" | "family";
  description: string;
}

export interface RPTimelineEntry {
  date: Date;
  event: string;
  description: string;
}
```

### 7.3 Roleplay Plugin (Simplified)

```typescript
// plugins/roleplay/index.ts

export default class RoleplayPlugin implements Plugin {
  private ctx!: PluginContext;
  private characters: Map<string, RPCharacter> = new Map();
  
  metadata(): PluginMetadata {
    return {
      id: "roleplay",
      name: "RolePlay & Lore System",
      version: "1.0.0",
      engine: ">=1.0.0",
      author: "SC Manager Team",
      description: "Manage character backgrounds and RP events",
      permissions: ["read-events", "storage-local", "ui-render"],
      ui: true
    };
  }
  
  async onLoad(ctx: PluginContext): Promise<void> {
    this.ctx = ctx;
    ctx.logger.info("RolePlay plugin loaded");
    
    // Load characters
    await this.loadCharacters();
    
    // Subscribe to events for automatic timeline updates
    ctx.eventStream.subscribe("MemberAdded", this.handleMemberAdded);
    ctx.eventStream.subscribe("OperationCompleted", this.handleOperationCompleted);
  }
  
  onEnable(): void {
    this.ctx.logger.info("RolePlay plugin enabled");
  }
  
  async onEvent(event: PluginEvent): Promise<void> {
    // Handled via subscriptions
  }
  
  onDisable(): void {
    this.ctx.logger.info("RolePlay plugin disabled");
  }
  
  // Plugin-specific methods
  async createCharacter(member_id: string, data: Omit<RPCharacter, "id" | "member_id">): Promise<RPCharacter> {
    const character: RPCharacter = {
      ...data,
      id: crypto.randomUUID(),
      member_id
    };
    
    this.characters.set(character.id, character);
    await this.ctx.storage.set(`character:${character.id}`, character);
    
    this.ctx.logger.info("Character created", { id: character.id });
    
    return character;
  }
  
  async getCharacter(member_id: string): Promise<RPCharacter | null> {
    return Array.from(this.characters.values())
      .find(c => c.member_id === member_id) ?? null;
  }
  
  async addTimelineEntry(
    character_id: string,
    event: string,
    description: string
  ): Promise<void> {
    const character = this.characters.get(character_id);
    if (!character) return;
    
    character.timeline.push({
      date: new Date(),
      event,
      description
    });
    
    await this.ctx.storage.set(`character:${character_id}`, character);
  }
  
  // Event handlers
  private handleMemberAdded = async (event: MemberAdded) => {
    this.ctx.logger.info("New member - could prompt for character creation", {
      member_id: event.member_id
    });
  };
  
  private handleOperationCompleted = async (event: OperationCompleted) => {
    // Auto-add to character timelines
    const operation = await this.ctx.readApi.query("getOperation", {
      id: event.operation_id
    });
    
    for (const participant of operation.participants) {
      const character = await this.getCharacter(participant.member_id);
      if (character) {
        await this.addTimelineEntry(
          character.id,
          `Operation: ${operation.name}`,
          `Participated in ${operation.operation_type} operation`
        );
      }
    }
  };
  
  private async loadCharacters(): Promise<void> {
    const keys = await this.ctx.storage.list("character:");
    for (const key of keys) {
      const character = await this.ctx.storage.get<RPCharacter>(key);
      if (character) {
        this.characters.set(character.id, character);
      }
    }
  }
}
```

---

## 8️⃣ WINDOWS INSTALLER (IDC-10)

### 8.1 Installer Requirements

```yaml
Platform: Windows 10/11 x64
Scope: User (no admin required)
Technology: WiX Toolset v4
Offline: Fully offline-capable
Signing: Code signing prepared
```

### 8.2 Installation Artifacts

```
Installation Directory:
C:\Users\<User>\AppData\Local\StarCitizenManager\
├── sc-manager.exe
├── resources/
├── config/
├── logs/
├── plugins/
│   ├── grinding/
│   └── roleplay/
└── updater/
```

### 8.3 Start Menu Integration (IDC-10)

```
Start Menu
└── Star Citizen Manager
   ├── Star Citizen Manager
   ├── Open Logs
   ├── Open Config
   ├── Plugin Manager (NEW)
   └── Uninstall
```

**Rules:**
- No auto-start without consent
- No hidden background services
- Clear naming, no vendor spam

### 8.4 Taskbar Integration

```rust
// infrastructure/installer/src/appusermodelid.rs

pub const APP_USER_MODEL_ID: &str = "StarCitizen.Manager.Desktop";

pub fn set_app_user_model_id() -> Result<()> {
    // Set Windows App User Model ID for proper taskbar grouping
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::Shell::*;
        unsafe {
            SetCurrentProcessExplicitAppUserModelID(
                windows::core::PCWSTR::from(APP_USER_MODEL_ID)
            )?;
        }
    }
    Ok(())
}
```

### 8.5 JumpList (IDC-10 Feature)

```rust
// infrastructure/installer/src/jumplist.rs

pub fn create_jumplist() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        // Create JumpList with max 5 entries
        let items = vec![
            JumpListItem {
                title: "Dashboard",
                path: "sc-manager://dashboard",
                icon_path: "resources/icons/dashboard.ico",
            },
            JumpListItem {
                title: "Operations",
                path: "sc-manager://operations",
                icon_path: "resources/icons/operations.ico",
            },
            JumpListItem {
                title: "Fleet Overview",
                path: "sc-manager://fleet",
                icon_path: "resources/icons/fleet.ico",
            },
            JumpListItem {
                title: "Grinding Tracker",
                path: "sc-manager://grinding",
                icon_path: "resources/icons/grinding.ico",
            },
            JumpListItem {
                title: "Settings",
                path: "sc-manager://settings",
                icon_path: "resources/icons/settings.ico",
            },
        ];
        
        // Register with Windows
        register_jumplist_items(items)?;
    }
    Ok(())
}
```

### 8.6 System Tray (Optional)

```rust
// Only active when background mode enabled
pub struct SystemTrayManager {
    status: SystemTrayStatus,
}

pub enum SystemTrayStatus {
    Online,
    Idle,
    Offline,
}

impl SystemTrayManager {
    pub fn set_status(&mut self, status: SystemTrayStatus) {
        let icon = match status {
            SystemTrayStatus::Online => "tray_online.ico",
            SystemTrayStatus::Idle => "tray_idle.ico",
            SystemTrayStatus::Offline => "tray_offline.ico",
        };
        
        self.update_icon(icon);
    }
    
    pub fn create_menu(&self) -> SystemTrayMenu {
        SystemTrayMenu {
            items: vec![
                MenuItem::Action("Open App", Action::OpenMainWindow),
                MenuItem::Separator,
                MenuItem::Status("Sync Status", self.get_sync_status()),
                MenuItem::Toggle("Pause Tracking", self.is_paused()),
                MenuItem::Separator,
                MenuItem::Action("Exit", Action::Quit),
            ]
        }
    }
}
```

### 8.7 Auto-Update (IDC-10 Safe)

```yaml
Policy:
  opt_in: true
  no_silent_updates: true
  changelog_visible: true

Flow:
  1. Check on startup
  2. Show dialog with changelog
  3. User confirms
  4. Download & verify
  5. Install on next restart
```

### 8.8 Uninstallation

```yaml
Behavior:
  - Fully reversible
  - User data optional

Options:
  - [ ] Keep configuration & logs
  - [ ] Keep plugins
  - [x] Remove everything (default)
```

### 8.9 WiX Configuration

```xml
<!-- installer/wix/main.wxs -->
<?xml version="1.0" encoding="UTF-8"?>
<Wix xmlns="http://wixtoolset.org/schemas/v4/wxs">
  <Product Id="*" 
           Name="Star Citizen Manager" 
           Language="1033" 
           Version="1.0.0" 
           Manufacturer="SC Manager Team"
           UpgradeCode="PUT-GUID-HERE">
    
    <Package InstallerVersion="500" 
             Compressed="yes"
             InstallScope="perUser" 
             Description="Star Citizen Organization Manager" />
    
    <MajorUpgrade DowngradeErrorMessage="A newer version is already installed." />
    
    <MediaTemplate EmbedCab="yes" />
    
    <!-- Features -->
    <Feature Id="ProductFeature" 
             Title="Star Citizen Manager" 
             Level="1">
      <ComponentGroupRef Id="ProductComponents" />
      <ComponentGroupRef Id="StartMenuShortcuts" />
      <ComponentGroupRef Id="PluginFiles" />
    </Feature>
    
    <!-- Install directory -->
    <Directory Id="TARGETDIR" Name="SourceDir">
      <Directory Id="LocalAppDataFolder">
        <Directory Id="INSTALLFOLDER" Name="StarCitizenManager" />
      </Directory>
      <Directory Id="ProgramMenuFolder">
        <Directory Id="ApplicationProgramsFolder" Name="Star Citizen Manager"/>
      </Directory>
    </Directory>
    
    <!-- Components -->
    <ComponentGroup Id="ProductComponents" Directory="INSTALLFOLDER">
      <Component Id="MainExecutable">
        <File Id="SCManagerEXE" 
              Source="$(var.BuildDir)\sc-manager.exe" 
              KeyPath="yes"
              Checksum="yes" />
        
        <!-- AppUserModelID -->
        <RegistryValue Root="HKCU" 
                       Key="Software\Classes\StarCitizen.Manager.Desktop"
                       Type="string" 
                       Value="StarCitizen.Manager.Desktop" 
                       KeyPath="no" />
      </Component>
      
      <!-- Resources, Config, etc -->
    </ComponentGroup>
    
    <!-- Start Menu -->
    <ComponentGroup Id="StartMenuShortcuts" Directory="ApplicationProgramsFolder">
      <Component Id="ApplicationShortcut">
        <Shortcut Id="ApplicationStartMenuShortcut"
                  Name="Star Citizen Manager"
                  Target="[INSTALLFOLDER]sc-manager.exe"
                  WorkingDirectory="INSTALLFOLDER"
                  Icon="SCManagerIcon.exe" />
        
        <Shortcut Id="OpenLogsShortcut"
                  Name="Open Logs"
                  Target="[INSTALLFOLDER]logs" />
        
        <Shortcut Id="OpenConfigShortcut"
                  Name="Open Config"
                  Target="[INSTALLFOLDER]config" />
        
        <Shortcut Id="PluginManagerShortcut"
                  Name="Plugin Manager"
                  Target="[INSTALLFOLDER]sc-manager.exe"
                  Arguments="--plugins" />
        
        <Shortcut Id="UninstallShortcut"
                  Name="Uninstall"
                  Target="[SystemFolder]msiexec.exe"
                  Arguments="/x [ProductCode]" />
        
        <RemoveFolder Id="CleanupStartMenuFolder" 
                      Directory="ApplicationProgramsFolder" 
                      On="uninstall"/>
        
        <RegistryValue Root="HKCU" 
                       Key="Software\SCManager" 
                       Name="installed" 
                       Type="integer" 
                       Value="1" 
                       KeyPath="yes"/>
      </Component>
    </ComponentGroup>
    
    <!-- Plugins -->
    <ComponentGroup Id="PluginFiles" Directory="INSTALLFOLDER">
      <Component Id="GrindingPlugin" Directory="PluginsFolder">
        <File Source="$(var.PluginsDir)\grinding\*" />
      </Component>
      <Component Id="RoleplayPlugin" Directory="PluginsFolder">
        <File Source="$(var.PluginsDir)\roleplay\*" />
      </Component>
    </ComponentGroup>
    
    <!-- Icons -->
    <Icon Id="SCManagerIcon.exe" SourceFile="$(var.BuildDir)\sc-manager.exe" />
    
    <!-- UI -->
    <UIRef Id="WixUI_Minimal" />
    <UIRef Id="WixUI_ErrorProgressText" />
    
  </Product>
</Wix>
```

### 8.10 Desktop Icon (Optional, Default OFF)

```xml
<Component Id="DesktopShortcut" Guid="PUT-GUID-HERE">
  <Condition>INSTALLDESKTOPSHORTCUT</Condition>
  <Shortcut Id="DesktopShortcut"
            Directory="DesktopFolder"
            Name="Star Citizen Manager"
            Target="[INSTALLFOLDER]sc-manager.exe"
            WorkingDirectory="INSTALLFOLDER"
            Icon="SCManagerIcon.exe" />
  <RegistryValue Root="HKCU" 
                 Key="Software\SCManager" 
                 Name="desktop_shortcut" 
                 Type="integer" 
                 Value="1" 
                 KeyPath="yes"/>
</Component>
```

---

## 9️⃣ IMPLEMENTATION STANDARDS

### 9.1 Error Handling
```rust
#[derive(Error, Debug)]
pub enum DomainError { /* ... */ }

#[derive(Error, Debug)]
pub enum ApplicationError { /* ... */ }

#[derive(Error, Debug)]
pub enum InfrastructureError { /* ... */ }

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Insufficient permissions: {0}")]
    InsufficientPermissions(String),
    
    #[error("Plugin execution timeout")]
    Timeout,
    
    #[error("Memory limit exceeded")]
    MemoryLimitExceeded,
}
```

### 9.2 Logging
```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(self))]
pub async fn load_plugin(&self, id: &str) -> Result<()> {
    info!(plugin_id = %id, "Loading plugin");
    // ...
}
```

### 9.3 Type Safety
```rust
pub struct OperationId(Uuid);
pub struct PluginId(String);
pub struct MissionId(String); // Plugin-internal
```

---

## 🔟 TESTING

### 10.1 Coverage Requirements
```yaml
Core_Domain:      100%
Application:      95%
Adapters:         85%
UI:               75%
Plugins:          85%  # NEW
Plugin_SDK:       100% # NEW
Overall:          85%
```

### 10.2 Plugin Testing
```typescript
// plugins/grinding/tests/mission.test.ts

describe("MissionEntity", () => {
  test("creates valid mission", () => {
    const result = MissionEntity.create({
      name: "Wikelo Delivery",
      category: "trade",
      npc_or_faction: "Wikelo",
      difficulty: 3,
      reputation_gain: 50,
      is_repeatable: true,
      metadata: {}
    });
    
    expect(result.ok).toBe(true);
  });
  
  test("rejects invalid difficulty", () => {
    const result = MissionEntity.create({
      name: "Invalid Mission",
      category: "combat",
      npc_or_faction: "Test",
      difficulty: 10, // ❌ Invalid
      reputation_gain: 50,
      is_repeatable: true,
      metadata: {}
    });
    
    expect(result.ok).toBe(false);
  });
});
```

---

## 1️⃣1️⃣ PERFORMANCE & SECURITY

### 11.1 Performance Budgets
```yaml
API: p95 ≤200ms, p99 ≤500ms
Event_Bus: ≤5ms publish
Desktop_UI: ≤1500ms load, ≤100ms interaction
Plugin_Load: ≤500ms
Plugin_Event_Handler: ≤100ms
```

### 11.2 Plugin Security
```yaml
Sandbox:
  memory_limit: 50MB
  cpu_time_limit: 1s
  no_network_access: true
  no_file_system: true
  no_core_access: true

Permissions:
  enforced_at: runtime
  escalation: forbidden
  audit: logged
```

---

## 1️⃣2️⃣ CHECKPOINTS

```yaml
CP0_Foundation:
  - [ ] Workspace + Turborepo + pnpm
  - [ ] Docker compose
  
CP1_Core_Domain:
  - [ ] All base aggregates (Org, Op, Member, Fleet, Diplomacy)
  - [ ] 100% test coverage
  - [ ] No tech dependencies
  
CP2_Plugin_SDK:
  - [ ] Plugin interface defined
  - [ ] Sandbox implementation
  - [ ] Permission system
  - [ ] Event distribution
  - [ ] Storage isolation
  - [ ] 100% test coverage
  
CP3_Grinding_Plugin:
  - [ ] Mission, GrindingGoal, MissionProgress entities
  - [ ] ToS-safe verification workflow
  - [ ] UI components
  - [ ] 85% test coverage
  - [ ] No gameplay automation
  
CP4_Roleplay_Plugin:
  - [ ] RPCharacter entity
  - [ ] Timeline system
  - [ ] UI components
  - [ ] 85% test coverage
  
CP5_Application_Layer:
  - [ ] All command/query handlers
  - [ ] Event publishing
  - [ ] 95% test coverage
  
CP6_Infrastructure:
  - [ ] Event bus
  - [ ] Postgres
  - [ ] DragonflyDB
  - [ ] Plugin runtime
  
CP7_Desktop_UI:
  - [ ] Tauri + SolidJS
  - [ ] shadcn/ui components
  - [ ] Plugin loader
  - [ ] 75% test coverage
  
CP8_Windows_Installer:
  - [ ] WiX configuration
  - [ ] Start Menu integration
  - [ ] JumpList
  - [ ] AppUserModelID
  - [ ] Uninstaller
  - [ ] IDC-10 compliant
  
CP9_Production_Ready:
  - [ ] All tests green
  - [ ] Performance budgets met
  - [ ] Security audit passed
  - [ ] Documentation complete
```

---

## 1️⃣3️⃣ DEFINITION OF DONE

### Code
```yaml
- [ ] Layer separation correct
- [ ] No business logic in adapters
- [ ] Events for all state changes
- [ ] Error handling complete
- [ ] No unwrap() in production
- [ ] Logging structured
- [ ] Plugins isolated (if plugin)
- [ ] No Core access from plugins (if plugin)
```

### Testing
```yaml
- [ ] Unit tests (domain ≥100%)
- [ ] Integration tests present
- [ ] E2E tests for critical flows
- [ ] Coverage ≥85%
- [ ] Plugin tests isolated
```

### Security
```yaml
- [ ] Input validation
- [ ] Authorization checks
- [ ] No secrets in code
- [ ] ToS compliance verified
- [ ] Plugin sandbox enforced
```

### Documentation
```yaml
- [ ] Public APIs documented
- [ ] Examples provided
- [ ] ADR created (if architectural)
- [ ] Plugin docs (if plugin)
```

### Windows Integration (Installer)
```yaml
- [ ] MSI builds correctly
- [ ] Start Menu integration
- [ ] Taskbar AppUserModelID
- [ ] JumpList works
- [ ] Uninstaller complete
- [ ] IDC-10 compliant
- [ ] No dark patterns
```

---

## 🔚 COPILOT FINAL INSTRUCTIONS

```
YOU ARE GITHUB COPILOT IMPLEMENTING SC MANAGER V6.

THIS VERSION INCLUDES:
✓ Complete Plugin SDK
✓ Grinding Plugin (ToS-safe)
✓ Roleplay Plugin (template)
✓ Windows Installer (IDC-10)

ABSOLUTE RULES:
1. Read this ENTIRE document before ANY code
2. Follow EVERY rule without interpretation
3. Plugins MUST be isolated (no Core access)
4. Grinding MUST NOT automate gameplay
5. Installer MUST be Windows-native (IDC-10)
6. Use EXACT tech stack
7. Respect ALL layer boundaries
8. STOP if uncertain

PLUGIN RULES (CRITICAL):
- Plugins CANNOT access Core directly
- Plugins CANNOT issue Commands
- Plugins ONLY subscribe to Events
- Plugins have scoped storage
- Sandbox limits ENFORCED
- No permission escalation

GRINDING RULES (ToS-CRITICAL):
- Manual reporting ONLY
- Officer verification REQUIRED
- NO automation
- NO auto-completion
- Read-only Game.log (future)

INSTALLER RULES (IDC-10):
- Windows-native behavior
- No auto-start without consent
- Clean uninstall
- Start Menu integration
- JumpList support
- AppUserModelID correct

BEFORE EVERY CODE GENERATION:
✓ Self-check protocol (12 points)
✓ Layer separation
✓ Plugin isolation (if plugin)
✓ ToS compliance (if grinding)
✓ Windows-native (if installer)

IF ANY CHECK FAILS:
→ STOP
→ Request clarification
→ Do NOT proceed

REMEMBER:
- Core = Pure business logic
- Application = Orchestration
- Plugins = Isolated extensions
- Grinding = ToS-safe tracking
- Installer = Windows-native

THIS DOCUMENT IS:
✓ Complete
✓ Unambiguous
✓ Non-negotiable
✓ Your single source of truth

DEVIATIONS = REJECTED

NOW PROCEED WITH CONFIDENCE.
```

---

**STATUS:** V6 FINAL & COMPLETE  
**VERSION:** 6.0.0  
**FEATURES:** Base + Plugin SDK + Grinding Plugin + Roleplay Plugin + Windows Installer  
**AUTHORITY:** ABSOLUTE  
**COMPLIANCE:** ToS-safe, IDC-10, Zero Deviation
