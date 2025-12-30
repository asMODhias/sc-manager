import { createSignal, createEffect } from "solid-js";

export function SettingsPanel(props: { plugin: any }) {
  const [optIn, setOptIn] = createSignal(false);

  createEffect(async () => {
    if (!props.plugin) return;
    const v = await props.plugin.isGamelogOptIn();
    setOptIn(v);
  });

  const toggle = async () => {
    if (!props.plugin) return;
    await props.plugin.setGamelogOptIn(!optIn());
    setOptIn(!optIn());
  };

  return (
    <div class="p-4">
      <h2 class="text-lg font-semibold">Game.log Parsing (Opt-in)</h2>
      <p class="text-sm text-muted mb-4">Parsing of local Game.log is optional and local-only; suggestions require manual verification by an officer.</p>
      <div class="flex items-center space-x-3">
        <label class="flex items-center space-x-2">
          <input type="checkbox" checked={optIn()} onChange={toggle} />
          <span>{optIn() ? 'Enabled' : 'Disabled'}</span>
        </label>
        <button class="btn" onClick={() => toggle()}>{optIn() ? 'Disable' : 'Enable'}</button>
      </div>
    </div>
  );
}
