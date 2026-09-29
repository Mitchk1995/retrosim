# Working on Retrosim

Read README.md, docs/DECISIONS.md, and docs/HANDOFF.md before changing the project.

The prototype must use Rust + Bevy. Primary display: 2560 x 1600. Artwork is PNG sprites/tile atlases rendered by Bevy; do not resume the superseded browser game or choose procedural Rust pixel painting as the production art architecture.

- Keep confirmed user decisions separate from recommendations. Combat is not chosen.
- Build and compare small playable demos before broad world or engine implementation.
- Preserve the approved expressive pixel-art direction. The earlier direct-drawn area's layout was rejected; do not copy it into the game.
- Use a square grid and consistent native pixel density. The approved image is a reference, not an exact 16x16 atlas.
- Keep the simulation authoritative for world state, events, inventory, abilities, and character memories. Model dialogue must not invent executed actions.
- UI work needs a rendered review at actual play scale. Make intentions, targeting, costs, and outcomes clear.
- Keep source-backed setting facts traceable; label invented demo content as placeholders.
- Avoid inventing final names for places, characters, abilities, or factions during prototypes.
- Do not add an AI provider, spend on inference, or assume subscription authentication without resolving the access/budget decision.
- Keep credentials, private conversations, and local saves out of this public repository.
- Prefer short feature branches and focused PRs after the initial repository setup. Include relevant verification and a visual/playable artifact when applicable.
- Do not claim a planned feature, CI check, protection rule, or demo is implemented until verified.
- Check changes with node tools/check-foundation.mjs. Add Rust checks when Rust code exists.

The user requested GPT-6.1 Sol implementation agents under the primary assistant's direction. Follow docs/IMPLEMENTATION_FLOW.md: scoped tasks, clear ownership, small useful parallelism, integration and final review by the primary assistant. This does not select an in-game NPC model.
