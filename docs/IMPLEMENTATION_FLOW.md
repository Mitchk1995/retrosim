# Implementation with GPT-6.1 Sol agents

The user requested GPT-6.1 Sol agents for implementation, with the primary assistant directing the work.

This is the coding workflow. The models that may eventually power game characters are a separate, unresolved decision.

## Responsibilities

The primary assistant owns the design, shared interfaces, acceptance criteria, integration, and final review. GPT-6.1 Sol agents handle bounded implementation or focused review tasks.

Start with one agent when work is sequential. Use a small number of agents in parallel only when tasks have independent files or interfaces agreed in advance. For example: one agent can implement input/action state while another implements a UI component against an agreed contract. Do not assign several agents the same files concurrently.

## Task brief

Every delegated task should specify:

- The exact player-facing outcome and relevant confirmed decisions.
- Files/directories the agent may change and interfaces it must preserve.
- What is outside scope.
- Acceptance criteria and appropriate checks.
- What to report: changes, verification, and any unresolved issue.

Give agents the minimum relevant context plus the repository instructions. Do not let an implementation task silently choose combat, canon, monetization, or model access.

## Handoff

1. Primary assistant defines the next small playable or visible result.
2. Sol agent implements within its scope or reports the concrete blocker.
3. Primary assistant inspects the diff, integrates it, and resolves cross-cutting decisions.
4. Run the relevant checks and inspect the actual UI/game result.
5. Show the user a concise playable/visual result and record the feedback.
6. Carry the agreed result into the next task.

Use additional agent review where it resolves a concrete integration or correctness risk. Avoid duplicate implementation, redundant testing, and repeated broad context.
