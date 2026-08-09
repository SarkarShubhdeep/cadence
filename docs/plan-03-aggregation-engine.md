# Slice 3 of 5 — Aggregation Engine

**Status:** Not started
**Depends on:** [`plan-02-capture-and-persistence.md`](plan-02-capture-and-persistence.md)
**Master plan:** [`master_plan.md`](master_plan.md)

## Goal

Turn raw `window_events` rows into the numbers the dashboard actually needs: per-app totals, focus sessions, and a context-switch count for "today." This slice is pure logic — testable without touching the OS or the UI.

## Scope

**In:**
- A focus-session heuristic (contiguous time in one app, above a minimum duration threshold — e.g. 30s — counts as a session)
- Aggregation functions: daily per-app totals, list of focus sessions, context-switch count
- Decide and document: compute aggregates on-the-fly from `window_events`, or persist to `daily_app_totals` as writes happen (pick the simpler option for MVP; on-the-fly is likely sufficient at this scale)
- Tauri commands exposing "today's aggregates" to the frontend
- Unit tests for the aggregation logic using fixture event data (no DB/OS dependency)

**Out:**
- Any UI rendering (slice 4)
- Multi-day history/trends (post-MVP per master plan)

## Tasks

- [ ] Define the focus-session threshold and switch-counting rule in code and in a short comment/doc note (why 30s, not 10s or 60s)
- [ ] Implement `compute_daily_app_totals(events) -> Vec<AppTotal>` as a pure function
- [ ] Implement `compute_focus_sessions(events) -> Vec<FocusSession>` as a pure function
- [ ] Implement `compute_context_switches(events) -> usize`
- [ ] Write unit tests covering: empty input, single app all day, rapid switching, sessions below threshold being excluded
- [ ] Expose a single Tauri command (e.g. `get_today_summary`) that reads events for today and returns all three aggregates together
- [ ] Commit as `feat: add aggregation engine for daily focus/switch metrics`

## Definition of Done

- Given a fixture set of events, tests assert the exact expected totals, sessions, and switch count.
- `get_today_summary` returns correct data end-to-end against the real SQLite file from slice 2.
- Aggregation functions have zero OS/DB dependencies (pure, unit-testable).

## Next

→ [`plan-04-dashboard-ui.md`](plan-04-dashboard-ui.md)
