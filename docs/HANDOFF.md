# Local Codex handoff

Checkpoint: 2026-09-29. The user explicitly asked to stop this Work session and continue in local Codex.

## Latest user decisions

- The prototype must use the game's **Rust + Bevy** stack, not a JavaScript/browser game.
- Primary display is **2560 x 1600**. Review native UI at this resolution; also retain usable smaller window layouts.
- Artwork should be **PNG sprites and tile atlases**. Rust + Bevy loads, renders and animates them. Do not spend time hand-coding every art pixel in Rust as the production graphics solution.
- Use GPT-6.1 Sol agents for scoped implementation, with the primary agent integrating and reviewing.
- Combat remains undecided. Compare act-to-advance and real-time-with-pause using the same encounter before expanding.

## What is actually done

- Public foundation is published on `main`; its GitHub Actions Foundation check passed.
- `crates/retrosim_sim` contains the dependency-free Rust simulation: a deterministic clearing, player and two companions, three enemies, previewed actions, costs/cooldowns, enemy intents, four companion orders, gathering, discovery, retreat, and defeat.
- The Rust simulation was compiled directly with `rustc --edition 2024 --test`; **all 12 tests passed**, including complete victories for seeds 1-3, deterministic replay, movement/collision/line of sight, cooldowns, guard, explicit interactions, and terminal states.
- `crates/retrosim_app` is an **unfinished native app checkpoint**, with Bevy 0.18.1 provisionally pinned. Dependency features and native integration still need compile verification.
- `src/lib.rs` records the shared controller/UI interface. `src/ui.rs` is the native UI draft. `src/pixels.rs` is a procedural reference-art port; retain it only if useful for exporting placeholder PNG assets or comparison. It is not the agreed final graphics architecture.
- The app entry point explicitly exits as unfinished. **There is no native playable build yet. No native screenshot, UI test, or Bevy build has passed.**
- `docs/workflows/rust.yml.example` is a proposed Windows build workflow, not an enabled or verified check.

## Continue here

1. Read `AGENTS.md`, this file, `docs/DECISIONS.md`, `docs/COMBAT_DEMOS.md` and `docs/UI.md`.
2. Confirm local Cargo can access crates.io, resolve dependencies, and commit the resulting Cargo.lock. Check that the pinned Bevy API/features match the code. Run formatting and simulation tests.
3. Finish the native app entry point and connect the shared simulation, native input/buttons, clocks, previews, reset, orders, and end states. Both modes must use the same state transitions. Real-time starts paused; queue one action, execute it each 0.85-second beat, and pause when window focus is lost. Invalid actions must not consume a step. These timings are prototype parameters.
4. Export or author a coherent small PNG atlas at consistent native pixel density and render it with Bevy. Preserve the approved reference direction; no new final names or lore. Separate authoritative simulation from rendering/UI.
5. Build and run the actual native Windows executable. Inspect at **2560 x 1600**, including readable UI scaling, crisp integer pixel scaling, paused targeting, crowded combat, orders, and terminal states. Capture real screenshots and a short recording.
6. Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo build -p retrosim_app`, and `node tools/check-foundation.mjs`. Enable the proposed Rust workflow only once its commands/package path are valid.
7. Publish the playable Windows build and ask which mode makes the user want another encounter. Do not add broader world systems or choose combat before feedback.

## Controller contract

- Simulation IDs: player 0, scout 1, warden 2, raider 3, archer 4, brute 5.
- `State::new`, `preview`, `advance`, `set_order`, `actor`, `is_walkable`, `find_path`, `has_line_of_sight`, `suggest_action`; inspect the crate for exact typed definitions.
- Guard blocks 3 damage per incoming hit for one beat. Strike deals 5 at range 1. Bolt deals 4 at range 5, costs 2 focus, and requires two intervening beats. Only Wait restores 1 focus. Potion restores up to 8 HP. Healing, damage and previews must match.
- `Interact(Some(target))` must act on exactly that object, never fall back to a nearby different object. Keyboard Interact may use `None` to select an adjacent available object.
- UI draft exports `layout_ui`, `update_ui`, and `ScreenLayout`; check its actual source at integration. The shared `Lab`, `Visuals`, `UiAction`, `Mode` and `Tool` types are in the app library.

## Superseded browser draft

A temporary browser version was built and tested before the user clarified Rust. It is **not the intended game implementation** and is not included in this branch. A local reference copy sits beside this checkout at `../retrosim-browser-draft/`; it includes the earlier visual screenshots and interaction draft if useful. Do not resume browser implementation or report browser tests as native validation.

## Why work moved

This ChatGPT Work session's terminal could not connect to crates.io or GitHub HTTPS. GitHub connector write access worked, but this did not grant terminal network access. Local Rust and Cargo are installed. The user chose to continue locally for direct dependency downloads, builds, and testing. No authentication changes, provider integration, or inference spending are needed for the combat lab.
