import { describe, it, expect } from "vitest";
import GrindingPlugin from "../src/index";

describe("GrindingPlugin - Opt-in and accept flow", () => {
  it("toggles opt-in and accepts suggestion", async () => {
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

    const plugin = new GrindingPlugin();
    await plugin.onLoad(mockCtx);

    // Initially opt-in false
    expect(await plugin.isGamelogOptIn()).toBe(false);

    // enable
    await plugin.setGamelogOptIn(true);
    expect(await plugin.isGamelogOptIn()).toBe(true);

    // simulate suggestion
    const suggestion = { id: "s3", mission_name: "Test", member_rsi: "G1", timestamp: "2025-12-27T00:00:00Z", raw_line: "" };
    await mockCtx._handler(suggestion);

    const stored = await mockCtx.storage.get("suggestion:s3");
    expect(stored).toBeDefined();

    // accept
    const progress = await plugin.acceptSuggestion("s3", "officer1");
    expect(progress).toBeDefined();
    expect(progress?.verification_state).toBe("pending");
  });
});