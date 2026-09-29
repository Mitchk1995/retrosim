# UI prototype brief

Status: design proposal; no UI prototype exists yet.

The first UI must help judge combat feel. It should answer: who am I controlling, what is happening, what can I do, and what will my choice cause?

## Shared layout

- World view receives most of the screen. Strong separation between walkable ground, obstacles, characters, and targeting overlays.
- Compact party strip: portrait, name/role placeholder, health, current activity, and a small number of meaningful conditions.
- Bottom action bar: basic action and a few abilities/items with readable costs, availability, and keyboard hints.
- Context panel on selection: target, intent, distance/range, effect preview, and one clearly marked confirmation.
- Persistent, unmistakable time control: stopped/running/resolve, with pause and reset.
- Short optional event log and concise hover details; avoid filling the screen with permanent text.

## Interaction states to prototype

No selection; hovered tile; selected character; valid/invalid target; queued action; resolving action; incoming threat; companion order acknowledged; defeat; encounter complete.

Important information should use labels or icons in addition to colour. UI scaling is independent of the art's integer pixel scale. Keep player and enemy silhouettes distinct from ground textures.

## Companion interaction

Clicking a companion should expose useful orders first during a fight. Outside combat, the same identity can open conversation, equipment, and shared memories later. Stable portraits, names, selection, and history connect those views.

A model request must not freeze ordinary controls. Implemented abilities and world facts remain authoritative even when conversational text is generated.

## Review method

Compare the same encounter in both first-phase combat modes. Inspect at 1280x720 and 1920x1080, including paused targeting and crowded action. Capture the real UI, not an unrelated image-generation mockup.

Review with the user before expanding menus, inventories, or simulation dashboards.
