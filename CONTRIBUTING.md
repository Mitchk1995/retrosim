# Development workflow

1. Write one small goal as an issue, with observable acceptance criteria.
2. Create a short branch: `demo/...`, `feat/...`, `fix/...`, or `art/...`.
3. Implement the smallest useful version. Keep design experiments easy to compare or replace.
4. Run checks appropriate to the change. Include a screenshot, short recording, or playable build for visible/interactive work.
5. Open a focused pull request. Explain the player benefit, what changed, what was actually verified, and what remains uncertain.
6. Merge once the result meets its acceptance criteria. Tag playable milestones and provide Windows builds when the game can build.

Keep main usable. GitHub discussions/reviews should support making the game; avoid administrative work without a practical benefit.

## Current gate

Run `node tools/check-foundation.mjs`. The Foundation workflow executes the same check on pushes and pull requests. It validates local documentation links, the demo comparison data, and the approved reference image.

## Gates to add with implementation

- Rust: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and Windows build.
- Simulation: meaningful state-transition, save/load, replay, and action-validation checks.
- UI/art: inspect the actual rendered view at usable window sizes; show hover/focus, selection, targeting, and empty/error states.
- Model integration: measure latency, inference spend, memory grounding, and behavior when responses arrive late or fail.

Do not add green checks that silently skip missing functionality. No branch protection or merge restrictions have been configured as part of this foundation.

## Public repository hygiene

Commit only project material. Keep keys, tokens, private chats, and saves outside version control. Existing fiction is referenced through source links and original project notes; do not upload sourcebooks or third-party asset packs without authorization.

## Agent workflow

The user requested GPT-6.1 Sol for scoped implementation, with the primary assistant directing, integrating, and reviewing. See [the implementation flow](docs/IMPLEMENTATION_FLOW.md).
