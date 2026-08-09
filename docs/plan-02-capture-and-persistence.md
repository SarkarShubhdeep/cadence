# Slice 2 of 5 — Rust Capture Loop & SQLite Persistence

**Status:** Not started
**Depends on:** [`plan-01-scaffold.md`](plan-01-scaffold.md)
**Master plan:** [`master_plan.md`](master_plan.md)

## Goal

A Rust background loop that detects active window/app changes on macOS and persists them as raw events to a local SQLite file. This is the "capture" half of the core loop — no aggregation or UI yet.

## Scope

**In:**
- macOS active-window/app detection (e.g. via `NSWorkspace`/Accessibility APIs, using a crate like `active-win-pos-rs` or a small custom FFI binding)
- A capture loop that emits `(app_name, window_title, started_at, ended_at)` on each app switch
- SQLite setup via `rusqlite`, with the `window_events` table from [`master_plan.md`](master_plan.md#8-data-model-mvp-sketch)
- Transactional writes so a crash mid-capture can't corrupt the DB
- A Tauri command to start/stop the capture loop manually

**Out:**
- Keystroke capture (explicitly out of MVP — see master plan)
- Aggregation/rollups (slice 3)
- Any UI beyond a manual start/stop trigger

## Tasks

- [ ] Evaluate and pick a macOS active-window crate (or write a minimal FFI wrapper); document the choice and why
- [ ] Implement the capture loop as its own Rust module, isolated from persistence (see [`.cursor/rules/rust.mdc`](../.cursor/rules/rust.mdc) on module boundaries)
- [ ] Add `rusqlite` dependency, create the SQLite file in the app's local data dir (via Tauri's path APIs)
- [ ] Write a migration/init step that creates `window_events` if it doesn't exist
- [ ] Wire the capture loop to write completed events transactionally
- [ ] Expose a Tauri command (`start_capture` / `stop_capture`) so the loop isn't always running silently
- [ ] Manual test: run for several minutes switching between apps, inspect the SQLite file with a DB browser (e.g. `sqlite3` CLI or DB Browser for SQLite) to confirm accuracy
- [ ] Confirm no raw keystrokes or full window content are ever written — only app name + window title
- [ ] Commit as `feat: add macOS window capture loop with SQLite persistence`

## Definition of Done

- Switching apps for a few minutes produces accurate, non-overlapping rows in `window_events`.
- Killing the app mid-capture leaves the DB in a valid state (no partial/corrupt rows).
- `cargo clippy -- -D warnings` passes; no `.unwrap()`/`.expect()` outside tests.

## Next

→ [`plan-03-aggregation-engine.md`](plan-03-aggregation-engine.md)
