import { MissionSuggestion, MissionProgress } from "./types";

export default class GrindingPlugin {
  private ctx: any;
  private suggestions: Map<string, MissionSuggestion> = new Map();

  metadata() {
    return {
      id: "grinding",
      name: "Mission Grinding Tracker",
      version: "1.0.0",
      engine: ">=1.0.0",
      author: "SC Manager Team",
      description: "Track mission grinding (ToS-safe, no automation)",
      permissions: ["read-events", "read-data", "storage-local", "ui-render"],
      ui: true,
    };
  }

  async onLoad(ctx: any) {
    this.ctx = ctx;
    this.ctx.logger?.info("Grinding plugin loaded (with gamelog support)");

    // Load persisted opt-in flag
    const optIn = (await this.ctx.storage.get<boolean>("gamelog:opt_in")) ?? false;
    if (optIn) {
      this.ctx.logger?.info("Game.log parsing opt-in enabled");
    }

    // Subscribe to mission suggestions emitted by the adapter
    try {
      this.ctx.eventStream.subscribe("MissionSuggestion", this.handleMissionSuggestion);
    } catch (err) {
      this.ctx.logger?.warn("Event subscription failed", { err });
    }

    // Load persisted suggestions
    const keys = await this.ctx.storage.list("suggestion:");
    for (const key of keys) {
      const s = await this.ctx.storage.get<MissionSuggestion>(key);
      if (s) this.suggestions.set(s.id, s);
    }
  }

  onEnable() {
    this.ctx.logger?.info("Grinding plugin enabled");
  }

  onDisable() {
    this.ctx.logger?.info("Grinding plugin disabled");
  }

  // Opt-in management
  async setGamelogOptIn(enabled: boolean): Promise<void> {
    await this.ctx.storage.set("gamelog:opt_in", enabled);
    this.ctx.logger?.info("Set gamelog opt-in", { enabled });
  }

  async isGamelogOptIn(): Promise<boolean> {
    return (await this.ctx.storage.get<boolean>("gamelog:opt_in")) ?? false;
  }

  // Event handler
  private handleMissionSuggestion = async (event: MissionSuggestion) => {
    // Enforce opt-in: ignore suggestions if opt-in false
    const optIn = await this.isGamelogOptIn();
    if (!optIn) return;

    // Store suggestion in plugin-scoped storage
    const key = `suggestion:${event.id}`;
    await this.ctx.storage.set(key, event);
    this.suggestions.set(event.id, event);
    this.ctx.logger?.info("Mission suggestion stored", { suggestion_id: event.id });
  };

  // Query suggestions
  async getSuggestions(): Promise<MissionSuggestion[]> {
    return Array.from(this.suggestions.values());
  }

  async acceptSuggestion(suggestionId: string, officerId: string): Promise<MissionProgress | null> {
    const s = this.suggestions.get(suggestionId);
    if (!s) return null;

    // Create a pending MissionProgress with verification_method 'log'
    const progress: MissionProgress = {
      id: cryptoRandomId(),
      mission_name: s.mission_name,
      member_id: s.member_rsi ?? "unknown",
      completions: 1,
      last_completed_at: new Date().toISOString(),
      verification_state: "pending",
      verification_method: "log",
    };

    // Persist
    await this.ctx.storage.set(`progress:${progress.id}`, progress);

    this.ctx.logger?.info("Suggestion accepted (pending verification)", { suggestionId, progress_id: progress.id });

    return progress;
  }
}

function cryptoRandomId() {
  // Minimal UUID-ish string for plugin internal use
  return "id-" + Math.random().toString(36).slice(2, 10) + Date.now().toString(36).slice(-4);
}