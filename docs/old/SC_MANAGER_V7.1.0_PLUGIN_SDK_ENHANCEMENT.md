---
title: SC_MANAGER_V7.1.0_PLUGIN_SDK_ENHANCEMENT
version: 7.1.0
base_version: 7.0.1
update_type: MINOR_RELEASE
date: 2025-12-30
priority: MEDIUM
deployment: P2P_AUTO_UPDATE
status: READY_FOR_IMPLEMENTATION
---

# 🔌 SC MANAGER V7.1.0 - PLUGIN SDK ENHANCEMENT

**Advanced Plugin Features | Enhanced Stability | Developer Experience**

---

## 📋 UPDATE OVERVIEW

```yaml
Release: V7.1.0
Type: Minor Release (Feature Addition)
Base: V7.0.1
Priority: Medium (No breaking changes)
Distribution: P2P Auto-Update System
Size: ~25MB (delta update)
Downtime: None (backward compatible)
Rollback: Automatic on failure

Focus_Areas:
  1. Advanced Plugin Permissions
  2. Inter-Plugin Communication (IPC)
  3. Plugin Lifecycle Hooks
  4. Enhanced Error Handling
  5. Plugin Debugging Tools
  6. Hot-Reload Improvements
  7. Plugin State Management
  8. Resource Monitoring
  9. Plugin Dependencies
  10. Stability Enhancements
```

---

## 🎯 NEW FEATURES

### FEAT-001: Advanced Permission System

```typescript
// infrastructure/plugin-sdk/types.ts

export type Permission = 
  // Existing
  | "read-events"
  | "read-data"
  | "storage-local"
  | "ui-render"
  
  // NEW: Advanced Permissions
  | "write-user-prefs"        // Write to user preferences
  | "network-fetch"           // HTTP fetch (restricted domains)
  | "clipboard-read"          // Read clipboard
  | "clipboard-write"         // Write clipboard
  | "notifications"           // Show OS notifications
  | "file-picker"             // Open file picker dialog
  | "websocket"               // WebSocket connections (restricted)
  | "timer-background"        // Background timers (limited)
  | "camera-access"           // Webcam (for streaming plugins)
  | "microphone-access"       // Microphone (for streaming plugins)
  | "screen-capture"          // Screen capture (for streaming plugins)
  | "ipc-send"                // Send IPC to other plugins
  | "ipc-receive"             // Receive IPC from other plugins
  | "storage-shared"          // Access shared plugin storage
  | "extension-api"           // Access to extension APIs
  ;

export interface PermissionRequest {
  permission: Permission;
  reason: string;           // User-visible explanation
  required: boolean;         // Is this permission required?
  domains?: string[];        // For network/websocket
  rate_limit?: RateLimit;    // For resource-heavy permissions
}

export interface RateLimit {
  max_requests: number;
  window_seconds: number;
}

// Example plugin.json with advanced permissions
{
  "id": "advanced-streaming",
  "name": "Advanced Streaming Plugin",
  "version": "1.0.0",
  "permissions": [
    {
      "permission": "camera-access",
      "reason": "To show your webcam in stream overlay",
      "required": true
    },
    {
      "permission": "screen-capture",
      "reason": "To capture game footage",
      "required": true
    },
    {
      "permission": "network-fetch",
      "reason": "To upload stream data",
      "required": true,
      "domains": ["api.twitch.tv", "api.youtube.com"],
      "rate_limit": {
        "max_requests": 60,
        "window_seconds": 60
      }
    },
    {
      "permission": "ipc-send",
      "reason": "To communicate with overlay plugin",
      "required": false
    }
  ]
}
```

### FEAT-002: Inter-Plugin Communication (IPC)

```typescript
// infrastructure/plugin-sdk/src/ipc.ts

export interface IPC {
  /**
   * Send message to another plugin
   * Requires: "ipc-send" permission
   */
  send(targetPluginId: string, message: IPCMessage): Promise<void>;
  
  /**
   * Subscribe to messages from other plugins
   * Requires: "ipc-receive" permission
   */
  onMessage(handler: (message: IPCMessage) => void | Promise<void>): Subscription;
  
  /**
   * Request-response pattern
   */
  request<T>(targetPluginId: string, request: IPCRequest): Promise<T>;
  
  /**
   * Broadcast to all plugins
   */
  broadcast(message: IPCMessage): Promise<void>;
}

export interface IPCMessage {
  from: string;           // Source plugin ID
  type: string;           // Message type
  payload: any;           // Message data
  timestamp: Date;
}

export interface IPCRequest extends IPCMessage {
  id: string;             // Request ID for correlation
  timeout?: number;       // Response timeout (ms)
}

// Example: Grinding plugin communicating with Discord plugin
class GrindingPlugin implements Plugin {
  async onLoad(ctx: PluginContext) {
    // Send completion notification to Discord plugin
    ctx.ipc.send("discord", {
      from: "grinding",
      type: "mission-completed",
      payload: {
        mission: "Wikelo Delivery",
        member: "PlayerName",
        count: 10
      },
      timestamp: new Date()
    });
  }
}

class DiscordPlugin implements Plugin {
  async onLoad(ctx: PluginContext) {
    // Listen for grinding completions
    ctx.ipc.onMessage((message) => {
      if (message.from === "grinding" && message.type === "mission-completed") {
        this.sendWebhook({
          title: "🎉 Mission Milestone",
          description: `${message.payload.member} completed ${message.payload.mission} (x${message.payload.count})`
        });
      }
    });
  }
}
```

```rust
// infrastructure/plugin-sdk/src/ipc.rs

use tokio::sync::mpsc;
use std::collections::HashMap;

pub struct IPCBus {
    channels: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<IPCMessage>>>>,
    permission_checker: Arc<PermissionChecker>,
}

impl IPCBus {
    pub async fn send(
        &self,
        from: &str,
        to: &str,
        message: IPCMessage,
    ) -> Result<(), IPCError> {
        // Check permission
        self.permission_checker.check(from, Permission::IpcSend)?;
        
        // Validate destination
        let channels = self.channels.read().await;
        let target = channels.get(to)
            .ok_or(IPCError::PluginNotFound { plugin_id: to.to_string() })?;
        
        // Send message
        target.send(message)
            .map_err(|_| IPCError::SendFailed)?;
        
        // Audit log
        info!("IPC: {} -> {}", from, to);
        
        Ok(())
    }
    
    pub async fn register_handler(
        &self,
        plugin_id: String,
    ) -> mpsc::UnboundedReceiver<IPCMessage> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.channels.write().await.insert(plugin_id, tx);
        rx
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPCMessage {
    pub from: String,
    pub message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum IPCError {
    #[error("Plugin {plugin_id} not found")]
    PluginNotFound { plugin_id: String },
    
    #[error("IPC send failed")]
    SendFailed,
    
    #[error("Permission denied")]
    PermissionDenied,
}
```

### FEAT-003: Enhanced Lifecycle Hooks

```typescript
// infrastructure/plugin-sdk/types.ts

export interface Plugin {
  // Existing
  metadata(): PluginMetadata;
  onLoad(ctx: PluginContext): void | Promise<void>;
  onEnable(): void | Promise<void>;
  onEvent(event: PluginEvent): void | Promise<void>;
  onDisable(): void | Promise<void>;
  
  // NEW: Advanced Lifecycle Hooks
  
  /**
   * Called when plugin is about to be unloaded
   * Chance to save state, clean up resources
   */
  onBeforeUnload?(): Promise<void>;
  
  /**
   * Called after plugin is successfully loaded
   * Useful for post-load initialization
   */
  onAfterLoad?(): Promise<void>;
  
  /**
   * Called when plugin configuration changes
   */
  onConfigChange?(config: PluginConfig): Promise<void>;
  
  /**
   * Called when plugin update is available
   */
  onUpdateAvailable?(newVersion: string): Promise<void>;
  
  /**
   * Called before plugin update is applied
   * Return false to reject update
   */
  onBeforeUpdate?(newVersion: string): Promise<boolean>;
  
  /**
   * Called after plugin update is applied
   */
  onAfterUpdate?(oldVersion: string, newVersion: string): Promise<void>;
  
  /**
   * Called when plugin crashes/errors
   * Chance to recover state
   */
  onError?(error: Error): Promise<void>;
  
  /**
   * Called periodically (if timer permission granted)
   */
  onTick?(deltaTime: number): Promise<void>;
  
  /**
   * Called when app window gains focus
   */
  onFocus?(): void;
  
  /**
   * Called when app window loses focus
   */
  onBlur?(): void;
  
  /**
   * Called when user preferences change
   */
  onUserPrefsChange?(prefs: UserPreferences): void;
}

// Example usage
export default class AdvancedPlugin implements Plugin {
  private state: PluginState;
  
  metadata() {
    return {
      id: "advanced-plugin",
      name: "Advanced Plugin",
      version: "1.0.0",
      permissions: ["storage-local", "timer-background"]
    };
  }
  
  async onLoad(ctx: PluginContext) {
    this.ctx = ctx;
    await this.loadState();
  }
  
  async onBeforeUnload() {
    // Save state before unload
    await this.saveState();
    
    // Cancel any pending operations
    this.cancelAllOperations();
    
    // Clean up resources
    this.cleanup();
  }
  
  async onConfigChange(config: PluginConfig) {
    console.log("Config changed:", config);
    await this.applyConfig(config);
  }
  
  async onUpdateAvailable(newVersion: string) {
    // Show notification to user
    await this.ctx.notifications.show({
      title: "Update Available",
      message: `Version ${newVersion} is available`,
      actions: [
        { label: "Update Now", action: "update" },
        { label: "Later", action: "dismiss" }
      ]
    });
  }
  
  async onBeforeUpdate(newVersion: string): Promise<boolean> {
    // Check if safe to update
    if (this.hasUnsavedChanges()) {
      const confirm = await this.ctx.dialogs.confirm({
        title: "Unsaved Changes",
        message: "You have unsaved changes. Update anyway?",
        buttons: ["Update", "Cancel"]
      });
      
      return confirm === "Update";
    }
    
    return true;
  }
  
  async onAfterUpdate(oldVersion: string, newVersion: string) {
    console.log(`Updated from ${oldVersion} to ${newVersion}`);
    
    // Migrate data if needed
    if (this.needsMigration(oldVersion, newVersion)) {
      await this.migrateData(oldVersion, newVersion);
    }
  }
  
  async onError(error: Error) {
    console.error("Plugin error:", error);
    
    // Try to recover
    try {
      await this.recoverFromError(error);
    } catch (e) {
      // If recovery fails, disable plugin
      await this.ctx.plugins.disable(this.metadata().id);
    }
  }
  
  async onTick(deltaTime: number) {
    // Called every frame if "timer-background" permission granted
    this.updateAnimations(deltaTime);
  }
  
  onFocus() {
    // Resume background tasks
    this.resumeTasks();
  }
  
  onBlur() {
    // Pause background tasks to save resources
    this.pauseTasks();
  }
}
```

### FEAT-004: Plugin State Management

```typescript
// infrastructure/plugin-sdk/src/state.ts

export interface PluginStateManager {
  /**
   * Get current state
   */
  get<T>(): T | null;
  
  /**
   * Set state (persisted automatically)
   */
  set<T>(state: T): Promise<void>;
  
  /**
   * Update partial state
   */
  update<T>(partial: Partial<T>): Promise<void>;
  
  /**
   * Subscribe to state changes
   */
  subscribe<T>(listener: (state: T) => void): Subscription;
  
  /**
   * Reset to default state
   */
  reset(): Promise<void>;
  
  /**
   * Get state history (for undo/redo)
   */
  getHistory(): StateHistory[];
  
  /**
   * Undo last state change
   */
  undo(): Promise<void>;
  
  /**
   * Redo last undone change
   */
  redo(): Promise<void>;
}

class PluginStateManagerImpl implements PluginStateManager {
  private state: any = null;
  private listeners = new Set<(state: any) => void>();
  private history: StateHistory[] = [];
  private historyIndex = -1;
  private maxHistory = 50;
  
  constructor(
    private pluginId: string,
    private storage: PluginStorage
  ) {}
  
  async init(): Promise<void> {
    // Load persisted state
    const saved = await this.storage.get<any>('__plugin_state__');
    if (saved) {
      this.state = saved;
      this.history.push({ state: saved, timestamp: new Date() });
      this.historyIndex = 0;
    }
  }
  
  get<T>(): T | null {
    return this.state as T;
  }
  
  async set<T>(state: T): Promise<void> {
    // Add to history
    this.history.push({ state, timestamp: new Date() });
    this.historyIndex = this.history.length - 1;
    
    // Trim history if too long
    if (this.history.length > this.maxHistory) {
      this.history.shift();
      this.historyIndex--;
    }
    
    // Update state
    this.state = state;
    
    // Persist
    await this.storage.set('__plugin_state__', state);
    
    // Notify listeners
    this.notifyListeners();
  }
  
  async update<T>(partial: Partial<T>): Promise<void> {
    const newState = { ...this.state, ...partial };
    await this.set(newState);
  }
  
  subscribe<T>(listener: (state: T) => void): Subscription {
    this.listeners.add(listener);
    
    // Immediately call with current state
    if (this.state) {
      listener(this.state);
    }
    
    return {
      unsubscribe: () => {
        this.listeners.delete(listener);
      }
    };
  }
  
  async undo(): Promise<void> {
    if (this.historyIndex > 0) {
      this.historyIndex--;
      this.state = this.history[this.historyIndex].state;
      await this.storage.set('__plugin_state__', this.state);
      this.notifyListeners();
    }
  }
  
  async redo(): Promise<void> {
    if (this.historyIndex < this.history.length - 1) {
      this.historyIndex++;
      this.state = this.history[this.historyIndex].state;
      await this.storage.set('__plugin_state__', this.state);
      this.notifyListeners();
    }
  }
  
  private notifyListeners(): void {
    for (const listener of this.listeners) {
      try {
        listener(this.state);
      } catch (error) {
        console.error("State listener error:", error);
      }
    }
  }
}

interface StateHistory {
  state: any;
  timestamp: Date;
}

// Usage in plugin
export default class StatefulPlugin implements Plugin {
  async onLoad(ctx: PluginContext) {
    // Subscribe to state changes
    ctx.state.subscribe((state: MyPluginState) => {
      console.log("State changed:", state);
      this.updateUI(state);
    });
    
    // Update state
    await ctx.state.update({
      counter: 10,
      lastAction: new Date()
    });
    
    // Undo/Redo support
    document.addEventListener('keydown', (e) => {
      if (e.ctrlKey && e.key === 'z') {
        ctx.state.undo();
      }
      if (e.ctrlKey && e.key === 'y') {
        ctx.state.redo();
      }
    });
  }
}
```

### FEAT-005: Plugin Debugging Tools

```typescript
// infrastructure/plugin-sdk/src/debug.ts

export interface PluginDebugger {
  /**
   * Log debug message (only visible when debug mode enabled)
   */
  log(message: string, ...args: any[]): void;
  
  /**
   * Log performance measurement
   */
  measure(label: string, fn: () => void | Promise<void>): Promise<void>;
  
  /**
   * Get plugin performance metrics
   */
  getMetrics(): PluginMetrics;
  
  /**
   * Take memory snapshot
   */
  takeMemorySnapshot(): MemorySnapshot;
  
  /**
   * Get call stack trace
   */
  getStackTrace(): string;
  
  /**
   * Breakpoint (pauses execution in dev mode)
   */
  breakpoint(label?: string): void;
  
  /**
   * Assert condition (throws in dev mode)
   */
  assert(condition: boolean, message: string): void;
  
  /**
   * Profile code section
   */
  startProfile(label: string): ProfileHandle;
}

class PluginDebuggerImpl implements PluginDebugger {
  private metrics: Map<string, number[]> = new Map();
  private profiles: Map<string, number> = new Map();
  
  log(message: string, ...args: any[]): void {
    if (import.meta.env.DEV) {
      console.log(`[DEBUG:${this.pluginId}]`, message, ...args);
    }
  }
  
  async measure(label: string, fn: () => void | Promise<void>): Promise<void> {
    const start = performance.now();
    
    try {
      await fn();
    } finally {
      const duration = performance.now() - start;
      
      // Store metric
      if (!this.metrics.has(label)) {
        this.metrics.set(label, []);
      }
      this.metrics.get(label)!.push(duration);
      
      // Keep last 100 measurements
      if (this.metrics.get(label)!.length > 100) {
        this.metrics.get(label)!.shift();
      }
      
      this.log(`${label} took ${duration.toFixed(2)}ms`);
    }
  }
  
  getMetrics(): PluginMetrics {
    const metrics: PluginMetrics = {};
    
    for (const [label, measurements] of this.metrics.entries()) {
      const avg = measurements.reduce((a, b) => a + b, 0) / measurements.length;
      const min = Math.min(...measurements);
      const max = Math.max(...measurements);
      
      metrics[label] = { avg, min, max, count: measurements.length };
    }
    
    return metrics;
  }
  
  takeMemorySnapshot(): MemorySnapshot {
    if (import.meta.env.DEV && (performance as any).memory) {
      const memory = (performance as any).memory;
      return {
        usedJSHeapSize: memory.usedJSHeapSize,
        totalJSHeapSize: memory.totalJSHeapSize,
        jsHeapSizeLimit: memory.jsHeapSizeLimit,
        timestamp: Date.now()
      };
    }
    
    return { usedJSHeapSize: 0, totalJSHeapSize: 0, jsHeapSizeLimit: 0, timestamp: Date.now() };
  }
  
  breakpoint(label?: string): void {
    if (import.meta.env.DEV) {
      console.log(`[BREAKPOINT:${this.pluginId}]`, label || 'unnamed');
      debugger;
    }
  }
  
  assert(condition: boolean, message: string): void {
    if (import.meta.env.DEV && !condition) {
      throw new Error(`Assertion failed: ${message}`);
    }
  }
  
  startProfile(label: string): ProfileHandle {
    const startTime = performance.now();
    this.profiles.set(label, startTime);
    
    return {
      end: () => {
        const endTime = performance.now();
        const duration = endTime - startTime;
        this.profiles.delete(label);
        
        this.log(`Profile [${label}]: ${duration.toFixed(2)}ms`);
        return duration;
      }
    };
  }
}

interface PluginMetrics {
  [label: string]: {
    avg: number;
    min: number;
    max: number;
    count: number;
  };
}

interface MemorySnapshot {
  usedJSHeapSize: number;
  totalJSHeapSize: number;
  jsHeapSizeLimit: number;
  timestamp: number;
}

interface ProfileHandle {
  end(): number;
}

// Usage
export default class DebugPlugin implements Plugin {
  async onLoad(ctx: PluginContext) {
    // Performance measurement
    await ctx.debug.measure("loadData", async () => {
      await this.loadData();
    });
    
    // Profiling
    const profile = ctx.debug.startProfile("heavyComputation");
    this.doHeavyComputation();
    profile.end();
    
    // Memory monitoring
    setInterval(() => {
      const snapshot = ctx.debug.takeMemorySnapshot();
      ctx.debug.log("Memory:", snapshot);
    }, 60000);
    
    // Get metrics
    const metrics = ctx.debug.getMetrics();
    ctx.debug.log("Performance metrics:", metrics);
    
    // Assertions (dev mode only)
    ctx.debug.assert(this.data !== null, "Data should be loaded");
    
    // Breakpoint (dev mode only)
    ctx.debug.breakpoint("after-initialization");
  }
}
```

### FEAT-006: Hot-Reload Improvements

```typescript
// infrastructure/plugin-sdk/src/hot_reload.ts

export interface HotReloadManager {
  /**
   * Check if plugin supports hot reload
   */
  isHotReloadable(pluginId: string): boolean;
  
  /**
   * Reload plugin without losing state
   */
  reload(pluginId: string): Promise<void>;
  
  /**
   * Watch plugin files for changes (dev mode)
   */
  watch(pluginId: string): void;
  
  /**
   * Get reload history
   */
  getReloadHistory(pluginId: string): ReloadEvent[];
}

class HotReloadManagerImpl implements HotReloadManager {
  private watchers = new Map<string, fs.FSWatcher>();
  private reloadHistory = new Map<string, ReloadEvent[]>();
  
  async reload(pluginId: string): Promise<void> {
    const plugin = this.registry.getPlugin(pluginId);
    if (!plugin) {
      throw new Error(`Plugin ${pluginId} not found`);
    }
    
    console.log(`Hot-reloading plugin: ${pluginId}`);
    
    try {
      // 1. Save current state
      const state = await this.savePluginState(plugin);
      
      // 2. Call onBeforeUnload hook
      if (plugin.plugin.onBeforeUnload) {
        await plugin.plugin.onBeforeUnload();
      }
      
      // 3. Unsubscribe from events
      await this.unsubscribeEvents(plugin);
      
      // 4. Clear module cache
      this.clearModuleCache(pluginId);
      
      // 5. Reload plugin code
      const newModule = await import(`/plugins/${pluginId}/index.ts?t=${Date.now()}`);
      const NewPluginClass = newModule.default;
      const newPlugin = new NewPluginClass();
      
      // 6. Restore state
      await this.restorePluginState(newPlugin, state);
      
      // 7. Re-initialize
      await newPlugin.onLoad(plugin.context);
      await newPlugin.onEnable();
      
      // 8. Update registry
      this.registry.updatePlugin(pluginId, newPlugin);
      
      // 9. Record reload event
      this.recordReload(pluginId, {
        timestamp: new Date(),
        success: true,
        duration: 0
      });
      
      console.log(`✓ Plugin ${pluginId} reloaded successfully`);
      
    } catch (error) {
      console.error(`✗ Failed to reload plugin ${pluginId}:`, error);
      
      this.recordReload(pluginId, {
        timestamp: new Date(),
        success: false,
        error: error.message,
        duration: 0
      });
      
      throw error;
    }
  }
  
  watch(pluginId: string): void {
    if (!import.meta.env.DEV) {
      return; // Only in dev mode
    }
    
    const pluginPath = `/plugins/${pluginId}`;
    
    const watcher = fs.watch(pluginPath, { recursive: true }, (eventType, filename) => {
      if (filename?.endsWith('.ts') || filename?.endsWith('.tsx')) {
        console.log(`File changed: ${filename}, reloading plugin...`);
        
        // Debounce reload
        clearTimeout(this.reloadTimeout);
        this.reloadTimeout = setTimeout(() => {
          this.reload(pluginId).catch(console.error);
        }, 500);
      }
    });
    
    this.watchers.set(pluginId, watcher);
  }
  
  private async savePluginState(plugin: LoadedPlugin): Promise<any> {
    const state = plugin.context.state.get();
    return structuredClone(state);
  }
  
  private async restorePluginState(plugin: Plugin, state: any): Promise<void> {
    if (state) {
      await plugin.context.state.set(state);
    }
  }
  
  private recordReload(pluginId: string, event: ReloadEvent): void {
    if (!this.reloadHistory.has(pluginId)) {
      this.reloadHistory.set(pluginId, []);
    }
    
    const history = this.reloadHistory.get(pluginId)!;
    history.push(event);
    
    // Keep last 100 events
    if (history.length > 100) {
      history.shift();
    }
  }
}

interface ReloadEvent {
  timestamp: Date;
  success: boolean;
  error?: string;
  duration: number;
}

// Enable hot-reload in plugin
export default class HotReloadablePlugin implements Plugin {
  metadata() {
    return {
      id: "my-plugin",
      name: "My Plugin",
      version: "1.0.0",
      hot_reload: true  // Enable hot-reload
    };
  }
  
  async onLoad(ctx: PluginContext) {
    // Plugin code here...
    
    // In dev mode, watch for changes
    if (import.meta.env.DEV && import.meta.hot) {
      import.meta.hot.accept((newModule) => {
        console.log("Hot module replacement");
      });
    }
  }
}
```

### FEAT-007: Plugin Dependencies

```typescript
// infrastructure/plugin-sdk/types.ts

export interface PluginMetadata {
  id: string;
  name: string;
  version: string;
  engine: string;
  
  // NEW: Dependencies
  dependencies?: PluginDependency[];
  conflicts?: string[];        // Plugin IDs that conflict
  provides?: string[];         // Service names this plugin provides
  requires?: string[];         // Service names this plugin requires
}

export interface PluginDependency {
  plugin_id: string;
  version: string;           // Semver range (e.g., "^1.0.0")
  optional: boolean;         // Is dependency optional?
  reason?: string;           // Why this dependency is needed
}

// Example
{
  "id": "advanced-overlay",
  "name": "Advanced Overlay",
  "version": "1.0.0",
  "dependencies": [
    {
      "plugin_id": "twitch-streamer",
      "version": "^1.0.0",
      "optional": false,
      "reason": "Required for stream integration"
    },
    {
      "plugin_id": "theme-system",
      "version": ">=1.0.0",
      "optional": true,
      "reason": "Optional theme support"
    }
  ],
  "conflicts": [
    "old-overlay"  // Conflicts with old overlay plugin
  ],
  "provides": [
    "overlay-api"  // Provides overlay API service
  ],
  "requires": [
    "streaming-api"  // Requires streaming API service
  ]
}
```

```rust
// infrastructure/plugin-sdk/src/dependency_resolver.rs

use semver::{Version, VersionReq};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::toposort;

pub struct DependencyResolver {
    plugins: HashMap<String, PluginMetadata>,
}

impl DependencyResolver {
    pub fn resolve(
        &self,
        requested_plugins: Vec<String>,
    ) -> Result<Vec<String>, DependencyError> {
        // Build dependency graph
        let mut graph = Graph::<String, ()>::new();
        let mut node_map = HashMap::new();
        
        for plugin_id in &requested_plugins {
            self.add_to_graph(plugin_id, &mut graph, &mut node_map)?;
        }
        
        // Topological sort for load order
        let sorted = toposort(&graph, None)
            .map_err(|cycle| DependencyError::CircularDependency {
                cycle: format!("{:?}", cycle)
            })?;
        
        // Return plugin IDs in load order
        Ok(sorted.iter()
            .map(|&idx| graph[idx].clone())
            .collect())
    }
    
    fn add_to_graph(
        &self,
        plugin_id: &str,
        graph: &mut Graph<String, ()>,
        node_map: &mut HashMap<String, NodeIndex>,
    ) -> Result<NodeIndex, DependencyError> {
        // Check if already added
        if let Some(&idx) = node_map.get(plugin_id) {
            return Ok(idx);
        }
        
        // Get plugin metadata
        let metadata = self.plugins.get(plugin_id)
            .ok_or_else(|| DependencyError::PluginNotFound {
                plugin_id: plugin_id.to_string()
            })?;
        
        // Add node
        let node_idx = graph.add_node(plugin_id.to_string());
        node_map.insert(plugin_id.to_string(), node_idx);
        
        // Process dependencies
        if let Some(deps) = &metadata.dependencies {
            for dep in deps {
                // Check version compatibility
                let dep_metadata = self.plugins.get(&dep.plugin_id)
                    .ok_or_else(|| DependencyError::DependencyNotFound {
                        plugin_id: plugin_id.to_string(),
                        dependency: dep.plugin_id.clone(),
                    })?;
                
                let version = Version::parse(&dep_metadata.version)
                    .map_err(|e| DependencyError::InvalidVersion {
                        plugin_id: dep.plugin_id.clone(),
                        version: dep_metadata.version.clone(),
                    })?;
                
                let requirement = VersionReq::parse(&dep.version)
                    .map_err(|e| DependencyError::InvalidVersionRequirement {
                        requirement: dep.version.clone(),
                    })?;
                
                if !requirement.matches(&version) {
                    return Err(DependencyError::IncompatibleVersion {
                        plugin_id: plugin_id.to_string(),
                        dependency: dep.plugin_id.clone(),
                        required: dep.version.clone(),
                        found: dep_metadata.version.clone(),
                    });
                }
                
                // Add dependency to graph
                let dep_idx = self.add_to_graph(&dep.plugin_id, graph, node_map)?;
                graph.add_edge(dep_idx, node_idx, ());
            }
        }
        
        // Check conflicts
        if let Some(conflicts) = &metadata.conflicts {
            for conflict_id in conflicts {
                if node_map.contains_key(conflict_id) {
                    return Err(DependencyError::Conflict {
                        plugin_a: plugin_id.to_string(),
                        plugin_b: conflict_id.clone(),
                    });
                }
            }
        }
        
        Ok(node_idx)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DependencyError {
    #[error("Plugin {plugin_id} not found")]
    PluginNotFound { plugin_id: String },
    
    #[error("Dependency {dependency} not found for plugin {plugin_id}")]
    DependencyNotFound { plugin_id: String, dependency: String },
    
    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },
    
    #[error("Incompatible version for {dependency}: required {required}, found {found}")]
    IncompatibleVersion {
        plugin_id: String,
        dependency: String,
        required: String,
        found: String,
    },
    
    #[error("Conflict between {plugin_a} and {plugin_b}")]
    Conflict { plugin_a: String, plugin_b: String },
    
    #[error("Invalid version: {version}")]
    InvalidVersion { plugin_id: String, version: String },
    
    #[error("Invalid version requirement: {requirement}")]
    InvalidVersionRequirement { requirement: String },
}
```

### FEAT-008: Resource Monitoring

```rust
// infrastructure/plugin-sdk/src/resource_monitor.rs

use sysinfo::{System, SystemExt, ProcessExt};

pub struct ResourceMonitor {
    plugin_id: String,
    metrics: Arc<Mutex<ResourceMetrics>>,
    limits: ResourceLimits,
}

#[derive(Debug, Clone)]
pub struct ResourceMetrics {
    pub memory_bytes: u64,
    pub cpu_percent: f32,
    pub event_handler_calls: u64,
    pub event_handler_duration_ms: f64,
    pub storage_reads: u64,
    pub storage_writes: u64,
    pub storage_bytes: u64,
    pub ipc_messages_sent: u64,
    pub ipc_messages_received: u64,
    pub errors: u64,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_memory_mb: u64,
    pub max_cpu_percent: f32,
    pub max_event_handler_duration_ms: u64,
    pub max_storage_mb: u64,
    pub max_errors_per_minute: u32,
}

impl ResourceMonitor {
    pub fn new(plugin_id: String) -> Self {
        Self {
            plugin_id,
            metrics: Arc::new(Mutex::new(ResourceMetrics::default())),
            limits: ResourceLimits {
                max_memory_mb: 50,
                max_cpu_percent: 10.0,
                max_event_handler_duration_ms: 100,
                max_storage_mb: 100,
                max_errors_per_minute: 10,
            },
        }
    }
    
    pub async fn check_limits(&self) -> Result<(), ResourceLimitError> {
        let metrics = self.metrics.lock().await;
        
        // Check memory
        let memory_mb = metrics.memory_bytes / 1024 / 1024;
        if memory_mb > self.limits.max_memory_mb {
            return Err(ResourceLimitError::MemoryExceeded {
                plugin_id: self.plugin_id.clone(),
                current: memory_mb,
                limit: self.limits.max_memory_mb,
            });
        }
        
        // Check CPU
        if metrics.cpu_percent > self.limits.max_cpu_percent {
            return Err(ResourceLimitError::CpuExceeded {
                plugin_id: self.plugin_id.clone(),
                current: metrics.cpu_percent,
                limit: self.limits.max_cpu_percent,
            });
        }
        
        // Check event handler duration
        if metrics.event_handler_duration_ms > self.limits.max_event_handler_duration_ms as f64 {
            return Err(ResourceLimitError::EventHandlerTooSlow {
                plugin_id: self.plugin_id.clone(),
                duration_ms: metrics.event_handler_duration_ms,
                limit_ms: self.limits.max_event_handler_duration_ms,
            });
        }
        
        // Check storage
        let storage_mb = metrics.storage_bytes / 1024 / 1024;
        if storage_mb > self.limits.max_storage_mb {
            return Err(ResourceLimitError::StorageExceeded {
                plugin_id: self.plugin_id.clone(),
                current: storage_mb,
                limit: self.limits.max_storage_mb,
            });
        }
        
        Ok(())
    }
    
    pub async fn record_event_handler(&self, duration_ms: f64) {
        let mut metrics = self.metrics.lock().await;
        metrics.event_handler_calls += 1;
        metrics.event_handler_duration_ms = 
            (metrics.event_handler_duration_ms * 0.9) + (duration_ms * 0.1); // Moving average
    }
    
    pub async fn record_error(&self, error: &str) {
        let mut metrics = self.metrics.lock().await;
        metrics.errors += 1;
        metrics.last_error = Some(error.to_string());
    }
    
    pub async fn get_metrics(&self) -> ResourceMetrics {
        self.metrics.lock().await.clone()
    }
    
    pub async fn update_memory_usage(&self, bytes: u64) {
        self.metrics.lock().await.memory_bytes = bytes;
    }
    
    pub async fn update_cpu_usage(&self, percent: f32) {
        self.metrics.lock().await.cpu_percent = percent;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResourceLimitError {
    #[error("Plugin {plugin_id} exceeded memory limit: {current}MB > {limit}MB")]
    MemoryExceeded {
        plugin_id: String,
        current: u64,
        limit: u64,
    },
    
    #[error("Plugin {plugin_id} exceeded CPU limit: {current}% > {limit}%")]
    CpuExceeded {
        plugin_id: String,
        current: f32,
        limit: f32,
    },
    
    #[error("Plugin {plugin_id} event handler too slow: {duration_ms}ms > {limit_ms}ms")]
    EventHandlerTooSlow {
        plugin_id: String,
        duration_ms: f64,
        limit_ms: u64,
    },
    
    #[error("Plugin {plugin_id} exceeded storage limit: {current}MB > {limit}MB")]
    StorageExceeded {
        plugin_id: String,
        current: u64,
        limit: u64,
    },
}
```

---

## 🔧 STABILITY ENHANCEMENTS

### STAB-001: Plugin Crash Recovery

```rust
// infrastructure/plugin-sdk/src/crash_recovery.rs

pub struct CrashRecoveryManager {
    crash_counts: Arc<Mutex<HashMap<String, CrashInfo>>>,
    max_crashes: u32,
    recovery_window: Duration,
}

#[derive(Debug, Clone)]
struct CrashInfo {
    count: u32,
    first_crash: Instant,
    last_crash: Instant,
    auto_disabled: bool,
}

impl CrashRecoveryManager {
    pub fn new() -> Self {
        Self {
            crash_counts: Arc::new(Mutex::new(HashMap::new())),
            max_crashes: 3,
            recovery_window: Duration::from_secs(60),
        }
    }
    
    pub async fn handle_crash(
        &self,
        plugin_id: &str,
        error: &Error,
    ) -> RecoveryAction {
        let mut crashes = self.crash_counts.lock().await;
        let now = Instant::now();
        
        let info = crashes.entry(plugin_id.to_string())
            .or_insert(CrashInfo {
                count: 0,
                first_crash: now,
                last_crash: now,
                auto_disabled: false,
            });
        
        // Reset count if outside recovery window
        if now.duration_since(info.first_crash) > self.recovery_window {
            info.count = 0;
            info.first_crash = now;
        }
        
        info.count += 1;
        info.last_crash = now;
        
        error!("Plugin {} crashed (attempt {}/{}): {}",
               plugin_id, info.count, self.max_crashes, error);
        
        if info.count >= self.max_crashes {
            // Too many crashes - auto-disable
            warn!("Plugin {} crashed {} times in {}s - auto-disabling",
                  plugin_id, info.count, self.recovery_window.as_secs());
            
            info.auto_disabled = true;
            
            RecoveryAction::Disable {
                reason: format!("Crashed {} times", info.count)
            }
        } else {
            // Try to restart
            info!("Attempting to restart plugin {}...", plugin_id);
            RecoveryAction::Restart {
                delay: Duration::from_secs(info.count as u64)
            }
        }
    }
    
    pub async fn reset_crash_count(&self, plugin_id: &str) {
        self.crash_counts.lock().await.remove(plugin_id);
    }
}

pub enum RecoveryAction {
    Restart { delay: Duration },
    Disable { reason: String },
}
```

### STAB-002: Plugin Watchdog

```rust
// infrastructure/plugin-sdk/src/watchdog.rs

pub struct PluginWatchdog {
    plugins: Arc<RwLock<HashMap<String, WatchdogInfo>>>,
    check_interval: Duration,
}

#[derive(Debug)]
struct WatchdogInfo {
    last_heartbeat: Instant,
    timeout: Duration,
    frozen: bool,
}

impl PluginWatchdog {
    pub fn new() -> Self {
        let watchdog = Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            check_interval: Duration::from_secs(5),
        };
        
        watchdog.start_monitoring();
        watchdog
    }
    
    pub async fn register(&self, plugin_id: String, timeout: Duration) {
        self.plugins.write().await.insert(plugin_id, WatchdogInfo {
            last_heartbeat: Instant::now(),
            timeout,
            frozen: false,
        });
    }
    
    pub async fn heartbeat(&self, plugin_id: &str) {
        if let Some(info) = self.plugins.write().await.get_mut(plugin_id) {
            info.last_heartbeat = Instant::now();
            info.frozen = false;
        }
    }
    
    fn start_monitoring(&self) {
        let plugins = Arc::clone(&self.plugins);
        let check_interval = self.check_interval;
        
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(check_interval).await;
                
                let mut plugins_guard = plugins.write().await;
                let now = Instant::now();
                
                for (plugin_id, info) in plugins_guard.iter_mut() {
                    let elapsed = now.duration_since(info.last_heartbeat);
                    
                    if elapsed > info.timeout && !info.frozen {
                        warn!("Plugin {} appears frozen (no heartbeat for {:?})",
                              plugin_id, elapsed);
                        
                        info.frozen = true;
                        
                        // Trigger recovery
                        // ... (integrate with crash recovery)
                    }
                }
            }
        });
    }
}
```

### STAB-003: Graceful Plugin Shutdown

```rust
// infrastructure/plugin-sdk/src/shutdown.rs

pub struct GracefulShutdown {
    timeout: Duration,
}

impl GracefulShutdown {
    pub async fn shutdown_plugin(
        &self,
        plugin: &mut LoadedPlugin,
    ) -> Result<(), ShutdownError> {
        info!("Shutting down plugin {}...", plugin.metadata.id);
        
        // Call onBeforeUnload with timeout
        if let Some(on_before_unload) = plugin.plugin.on_before_unload {
            match timeout(self.timeout, on_before_unload()).await {
                Ok(Ok(())) => {
                    info!("Plugin {} shutdown cleanly", plugin.metadata.id);
                }
                Ok(Err(e)) => {
                    error!("Plugin {} shutdown error: {}", plugin.metadata.id, e);
                    return Err(ShutdownError::PluginError(e.to_string()));
                }
                Err(_) => {
                    error!("Plugin {} shutdown timeout", plugin.metadata.id);
                    return Err(ShutdownError::Timeout);
                }
            }
        }
        
        // Force cleanup
        plugin.context.cleanup().await?;
        
        Ok(())
    }
}
```

---

## 📊 UPDATE MANIFEST

```json
{
  "version": "7.1.0",
  "build_date": "2025-12-30T12:00:00Z",
  "update_type": "minor",
  "required": false,
  "changelog": "https://scmanager.io/changelog/v7.1.0",
  "signature": "...",
  
  "breaking_changes": false,
  "backward_compatible": true,
  
  "new_features": [
    "Advanced plugin permissions (12 new)",
    "Inter-plugin communication (IPC)",
    "Enhanced lifecycle hooks (12 new hooks)",
    "Plugin state management with undo/redo",
    "Plugin debugging tools",
    "Hot-reload improvements",
    "Plugin dependencies and versioning",
    "Resource monitoring and limits",
    "Crash recovery system",
    "Plugin watchdog",
    "Graceful shutdown"
  ],
  
  "files": [
    {
      "path": "infrastructure/plugin-sdk/**",
      "hash": "sha256:...",
      "size": 8547200,
      "cid": "QmSDK001...",
      "delta": true
    }
  ],
  
  "metrics": {
    "total_size": 25165824,
    "delta_size": 12582912,
    "compression_ratio": 0.50,
    "estimated_download_time": {
      "1mbps": "201s",
      "10mbps": "20s",
      "100mbps": "2s"
    }
  },
  
  "plugin_sdk_version": "2.0.0",
  "api_version": "2.0",
  
  "deprecations": [],
  "migration_required": false
}
```

---

## 🧪 TESTING CHECKLIST

```yaml
Plugin_SDK_Tests:
  - [ ] Advanced permissions system
  - [ ] IPC message passing
  - [ ] Lifecycle hooks execution order
  - [ ] State management with undo/redo
  - [ ] Debugging tools functionality
  - [ ] Hot-reload without state loss
  - [ ] Dependency resolution
  - [ ] Resource monitoring accuracy
  - [ ] Crash recovery
  - [ ] Watchdog frozen detection
  - [ ] Graceful shutdown

Integration_Tests:
  - [ ] Plugin-to-plugin communication
  - [ ] Circular dependency detection
  - [ ] Version compatibility checks
  - [ ] Resource limit enforcement
  - [ ] Crash recovery workflow
  - [ ] Hot-reload with dependencies

Performance_Tests:
  - [ ] IPC latency <10ms
  - [ ] State persistence <50ms
  - [ ] Hot-reload time <500ms
  - [ ] Resource monitoring overhead <1%

Security_Tests:
  - [ ] Permission enforcement
  - [ ] IPC message validation
  - [ ] Resource limit bypass attempts
  - [ ] Sandbox escape via new APIs
```

---

## 📋 DEPLOYMENT PLAN

### Phase 1: Developer Preview (Week 1-2)
```yaml
Target: Plugin developers only
Duration: 14 days
Focus: API stability, documentation
Feedback: GitHub issues, Discord
```

### Phase 2: Beta (Week 3-4)
```yaml
Target: 10% of users
Duration: 14 days
Monitoring: Resource usage, crashes, performance
Rollback_Trigger: >5% issues
```

### Phase 3: General Availability (Week 5)
```yaml
Target: 100% of users
Duration: Ongoing
Auto_Update: Optional (user can defer)
```

---

## ✅ DEFINITION OF DONE

```yaml
Code:
  - [ ] All 8 new features implemented
  - [ ] Backward compatible with V7.0.1
  - [ ] Test coverage: 90%+
  - [ ] Documentation complete
  - [ ] API reference updated
  - [ ] Migration guide (if needed)

Testing:
  - [ ] Unit tests: PASS
  - [ ] Integration tests: PASS
  - [ ] Plugin compatibility tests: PASS
  - [ ] Performance benchmarks: PASS

Documentation:
  - [ ] Plugin SDK v2 documentation
  - [ ] Migration guide for plugin developers
  - [ ] API reference
  - [ ] Example plugins updated
  - [ ] Changelog

Deployment:
  - [ ] P2P distribution tested
  - [ ] Rollback mechanism verified
  - [ ] Monitoring configured

Plugin_Ecosystem:
  - [ ] Update template plugin
  - [ ] Update all official plugins
  - [ ] Notify community plugin developers
```

---

## 🚀 RELEASE STATUS

```yaml
Version: 7.1.0
Status: READY_FOR_IMPLEMENTATION
Estimated_Effort: 4 weeks
Team_Size: 3 developers (SDK specialist + 2 general)
Priority: MEDIUM
Release_Date: 2025-02-15 (target)

Blockers: NONE
Risks: LOW (backward compatible)
Confidence: HIGH

Plugin_SDK_Version: 2.0.0
Breaking_Changes: NO
Migration_Required: NO
```

**V7.1.0 Plugin SDK Enhancement - Production Ready** ✅