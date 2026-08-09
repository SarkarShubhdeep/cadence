# Slice 2 of 5 — Rust Capture Loop & SQLite Persistence

**Status:** Done
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

- [x] Evaluate and pick a macOS active-window crate (or write a minimal FFI wrapper); document the choice and why — chose [`active-win-pos-rs`](https://crates.io/crates/active-win-pos-rs) (small, actively maintained, simple `get_active_window() -> Result<ActiveWindow, ()>` API); polls at 1s intervals since the crate has no event-driven API
- [x] Implement the capture loop as its own Rust module, isolated from persistence (see [`.cursor/rules/rust.mdc`](../.cursor/rules/rust.mdc) on module boundaries) — [`src-tauri/src/capture.rs`](../src-tauri/src/capture.rs)
- [x] Add `rusqlite` dependency, create the SQLite file in the app's local data dir (via Tauri's path APIs) — [`src-tauri/src/db.rs`](../src-tauri/src/db.rs), opened in `setup()` via `app.path().app_data_dir()`
- [x] Write a migration/init step that creates `window_events` if it doesn't exist
- [x] Wire the capture loop to write completed events transactionally — one event is inserted only when a window/app span closes (on switch or stop); no in-progress rows are ever written, so a crash only loses the unfinished span
- [x] Expose a Tauri command (`start_capture` / `stop_capture`) so the loop isn't always running silently — plus `is_capturing` for the UI — [`src-tauri/src/commands.rs`](../src-tauri/src/commands.rs), wired to Start/Stop buttons in [`src/App.tsx`](../src/App.tsx)
- [x] Manual test: run for several minutes switching between apps, inspect the SQLite file with a DB browser (e.g. `sqlite3` CLI or DB Browser for SQLite) to confirm accuracy — verified by scripted runs plus manual app usage; `window_events` populated correctly
- [x] Confirm no raw keystrokes or full window content are ever written — only app name + window title (empty titles, e.g. without Screen Recording permission, are stored as `NULL` rather than an empty string)
- [x] Commit as `feat: add macOS window capture loop with SQLite persistence`

## Definition of Done

- Switching apps for a few minutes produces accurate, non-overlapping rows in `window_events`.
- Killing the app mid-capture leaves the DB in a valid state (no partial/corrupt rows).
- `cargo clippy -- -D warnings` passes; no `.unwrap()`/`.expect()` outside tests.

## Next

→ [`plan-03-aggregation-engine.md`](plan-03-aggregation-engine.md)
