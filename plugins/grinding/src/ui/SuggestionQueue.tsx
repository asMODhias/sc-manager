import { createSignal, createEffect, For } from "solid-js";

export function SuggestionQueue(props: { plugin: any }) {
  const [suggestions, setSuggestions] = createSignal([] as any[]);

  createEffect(async () => {
    if (!props.plugin) return;
    const optIn = await props.plugin.isGamelogOptIn();
    if (!optIn) {
      setSuggestions([]);
      return;
    }
    const s = await props.plugin.getSuggestions();
    setSuggestions(s);
  });

  const handleAccept = async (id: string) => {
    if (!props.plugin) return;
    await props.plugin.acceptSuggestion(id, currentOfficerId);
    // Refresh
    const s = await props.plugin.getSuggestions();
    setSuggestions(s);
  };

  const handleEnableOptIn = async () => {
    if (!props.plugin) return;
    await props.plugin.setGamelogOptIn(true);
    const s = await props.plugin.getSuggestions();
    setSuggestions(s);
  };
  return (
    <div class="p-4">
      <h2 class="text-lg font-semibold">Game.log Suggestions</h2>
      <div class="space-y-2">
        <For each={suggestions()}>
          {(s: any) => (
            <div class="p-3 border rounded flex justify-between items-center">
              <div>
                <div class="font-medium">{s.mission_name}</div>
                <div class="text-sm text-muted">Reported by {s.member_rsi ?? 'unknown'} at {new Date(s.timestamp).toLocaleString()}</div>
              </div>
              <div>
                <button class="btn" onClick={() => handleAccept(s.id)}>Import as Progress</button>
              </div>
            </div>
          )}
        </For>
      </div>
    </div>
  );
}
