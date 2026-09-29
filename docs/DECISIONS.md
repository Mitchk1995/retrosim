# Decision log

Last updated: 2026-09-29.

## Confirmed

- The player is a persistent adventurer: exploring, fighting, gathering, and recruiting capable companions.
- Companion relationships and interactive conversations are central.
- A living society, observable emergence, long-term progression, broad possibilities, and meaningful mysteries are goals.
- Use a flat square grid with structured, consistent, reusable assets.
- The generated character-and-tile sheet approved on September 29 is the visual reference: expressive characters, chunky pixels, and rich colours.
- The earlier directly drawn area layout is not approved.
- Use documented, existing fiction as the basis of the world.
- The repository is public at Mitchk1995/retrosim.
- Compare demos before building a large amount of game or simulation code.
- Build a good, clear UI as part of those demos, not as an afterthought.
- Use GPT-6.1 Sol agents for scoped implementation, with the primary assistant directing, integrating, and reviewing.

## Proposed, not confirmed

- Rust + Bevy; Windows desktop first.
- Dalelands/Cormanthor and Myth Drannor as a Forgotten Realms starting region. Choose a source-backed era before writing canonical content.
- A small travelling party with autonomous companions.
- Recoverable defeat as the initial default.

## Open

| Choice | How to resolve it |
| --- | --- |
| Combat and pacing | Play comparable small demos; do not choose on description alone. |
| Party size and control | Prototype player control plus companion policies/occasional orders. |
| Defeat, retreat, saves, permanent loss | Decide after the first combat comparison; demos reset freely. |
| Setting and era | Confirm the preferred world and gather authoritative source references. |
| Model access and budget | Verify the supported authentication/provider and any spending cap before integration. |
| Tile resolution | Translate a small approved subset to consistent native pixels and inspect it at play scale. |

ChatGPT subscription access for a custom game's inference has not been established. No provider or paid usage is authorized by this document.

## Keep separate

The approved image establishes style; it does not approve a final world layout, character roster, or lore. The initial four visual character roles remain placeholders. A combat demo is an experiment, not a promise that its mechanics will remain in the finished game.
