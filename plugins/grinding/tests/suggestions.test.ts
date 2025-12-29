import { describe, it, expect, vi } from "vitest";
import GrindingPlugin from "../src/index";

describe("GrindingPlugin - MissionSuggestion handling", () => {
  it("stores suggestion when opt-in is enabled", async () => {
    // Mock context
    const storage = new Map<string, any>();
    const mockCtx: any = {
      storage: {
        get: async (k: string) => storage.get(k),
        set: async (k: string, v: any) => storage.set(k, v),
        list: async (prefix?: string) => Array.from(storage.keys()).filter(k => k.startsWith(prefix ?? "")),
      },
      eventStream: {
        subscribe: (t: string, h: any) => {
          // Expose handler
          mockCtx._handler = h;
        },
      },
      logger: { info: () => {}, warn: () => {} },
    };

    // enable opt-in
    await mockCtx.storage.set("gamelog:opt_in", true);

    const plugin = new GrindingPlugin();
    await plugin.onLoad(mockCtx);

    // simulate event
    const suggestion = {
      id: "s1",
      mission_name: "Wikelo Delivery",
      member_rsi: "Alpha_One",
      timestamp: "2025-12-27T20:34:12Z",
      raw_line: "...",
    };

    await mockCtx._handler(suggestion);

    const stored = await mockCtx.storage.get("suggestion:s1");
    expect(stored).toBeDefined();
    expect(stored.mission_name).toBe("Wikelo Delivery");
  });

  it("ignores suggestion when opt-in is disabled", async () => {
    const storage = new Map<string, any>();
    const mockCtx: any = {
      storage: {
        get: async (k: string) => storage.get(k),
        set: async (k: string, v: any) => storage.set(k, v),
        list: async (prefix?: string) => Array.from(storage.keys()).filter(k => k.startsWith(prefix ?? "")),
      },
      eventStream: {
        subscribe: (t: string, h: any) => {
          mockCtx._handler = h;
        },
      },
      logger: { info: () => {}, warn: () => {} },
    };

    // opt-in false (default)
    const plugin = new GrindingPlugin();
    await plugin.onLoad(mockCtx);

    const suggestion = {
      id: "s2",
      mission_name: "Mining Run",
      member_rsi: "Beta",
      timestamp: "2025-12-27T21:00:00Z",
      raw_line: "...",
    };

    await mockCtx._handler(suggestion);

    const stored = await mockCtx.storage.get("suggestion:s2");
    expect(stored).toBeUndefined();
  });
});