# Desktop Integration Guide — Game.log Suggestions (Grinding Plugin)

Purpose: How to integrate the `grinding` plugin's Game.log suggestion UI and opt-in settings into the Tauri + SolidJS desktop application.

Guidelines (must follow COPILOT doc rules):
- Keep work in UI layer only (no business logic in app)
- Expose plugin UI routes and menu items only when plugin is enabled and has `ui-render` permission
- Ensure privacy & opt-in copy is visible before enabling parsing

Suggested steps:

1. Router

Add routes to your SolidJS router (example using a simple routes map):

```ts
// src/routes.ts
import GrindingDashboard from "@/plugins/grinding/GrindingDashboard";
import SuggestionQueue from "@/plugins/grinding/SuggestionQueue";
import SettingsPanel from "@/plugins/grinding/SettingsPanel";

export const routes = [
  { path: "/grinding", component: GrindingDashboard },
  { path: "/grinding/suggestions", component: SuggestionQueue },
  { path: "/grinding/settings", component: SettingsPanel },
];
```

2. Menu / Settings

Add a settings entry and a menu link to access *Game.log Parsing* under the Grinding plugin settings. Ensure that the SettingsPanel is used to toggle opt-in and that toggling calls `plugin.setGamelogOptIn(true|false)`.

3. Plugin Bridge / Context

Ensure `window.pluginRegistry.get("grinding")` returns the plugin object in the renderer. Use the plugin context to call `isGamelogOptIn`, `getSuggestions`, and `acceptSuggestion`.

4. Visibility & Permission checks

Before showing the SuggestionQueue, confirm `plugin.metadata().permissions` includes `storage-local` and `read-events`, and that `isGamelogOptIn()` returns `true`.

5. Privacy and Opt-in Copy

Use the following copy (or adapt):
> "Game.log parsing is local-only and opt-in. The parser runs on your machine and suggests mission completions for manual verification by an officer. No data leaves your PC without explicit consent."

6. Tests

Add E2E tests that verify:
- Opt-in toggle changes behavior (suggestions appear only when enabled)
- Accepting a suggestion creates pending progress and appears in the VerificationQueue

7. Release notes / messaging

Document the feature in release notes and ensure the installer/upgrade flow does not enable parsing by default (opt-in must be user-driven).

---

If you want, I can also add a small PR patch that adds a sample router registration file and a menu item placeholder in the desktop app repo (if available). Otherwise, include this doc in the PR so reviewers know how to integrate the UI.
