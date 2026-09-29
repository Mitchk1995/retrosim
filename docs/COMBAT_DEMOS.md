# Combat comparison lab

Status: specification only. No combat demo is implemented yet.

The purpose is to discover what feels good before building a large game. Use the same party, readable art, small environment, goals, and approximate difficulty. Reset instantly and play each candidate in a few minutes. Preserve consistent core UI while exposing each mode's clock clearly.

## Four candidates

| Mode | What the player actually does | What it could make enjoyable | Main question |
| --- | --- | --- | --- |
| World moves when you act | Take one grid step or choose an ability; then creatures and autonomous companions take their actions. | Explore at your own pace, inspect threats, and make decisions without a reflex timer. | Does one action per world beat feel satisfying or too stop-start? |
| Real-time with pause | Control your character while companions fight autonomously; pause to aim, issue an order, or change priorities. | Watch the party cooperate in a living scene while retaining time to think. | Can the UI make the action readable without constant pausing? |
| Plan and resolve | Queue a short intent for your character, optionally give a companion order, then watch a short simultaneous beat resolve. | Predict enemy moves and enjoy coordinated party actions. | Does simultaneous resolution feel clever and fair, or unpredictable? |
| Party commands with timing | Choose attacks, spells, or items, then optionally time a hit/block input; characters handle movement. | Expressive party combat with satisfying execution and little positioning work. | Are the timing moments fun enough, and do fights fit naturally into the travelling world? |

These are proposed mechanics, not descriptions of existing implemented games.

## Order of work

1. Shared arena, target selection, information hierarchy, input feedback, and reset.
2. World-moves-when-you-act demo.
3. Real-time-with-pause demo in the same arena.
4. Compare those two with the user.
5. Build plan-and-resolve or party-timing only if the comparison leaves a useful unanswered question.

The phase-one pair shares grid actions and autonomous companion policies. Do not build four separate combat engines up front.

## Shared encounter

Use an unnamed trail clearing with a ruined gateway, a few obstacles, a visible side route, one gatherable resource, and a clear retreat exit. Design the space deliberately; do not reuse the rejected art-study layout.

Start with the player and two companions, three enemies with distinct readable roles, and one objective: reach and investigate a sealed marker, then leave with the discovery. Demo counts are test parameters, not a final party-size decision.

Use a deterministic scenario seed and offer a variation button after the first comparison. Keep enemy and companion decision logic local and simple so model latency and dialogue quality do not confound combat feel.

## Controls and presentation

- Keyboard and mouse first; list controls inside the demo.
- Show whether time is stopped, running, or resolving.
- Make queued actions and their cancellation visible.
- Show valid targets, range, cost, and a concise expected effect before confirmation.
- Telegraph enemy intent where the rules make it knowable; distinguish uncertainty from guaranteed outcomes.
- Let companions choose routine movement/attacks. Offer focus target, protect, regroup, and withdraw.
- Keep action feedback adjacent to the world, with an optional event log.
- Pause/reset should be immediate. No progression grind or irreversible losses in a comparison.

## Acceptance criteria

- A new player can identify who they control, the objective, the current time mode, and the next available action.
- The scene can be reset without restarting the app.
- The player can inspect a threat, give a companion order, use an ability, and retreat.
- Damage, effects, cooldowns, and resource changes match the visible state.
- The same scenario is playable in the first two modes with comparable numbers and shared assets.
- Capture a short recording and a playable Windows build when implemented.
- Ask which version makes the user want another encounter; record reasons before expanding.
