# Slice 4 of 5 — Dashboard UI

**Status:** Not started
**Depends on:** [`plan-03-aggregation-engine.md`](plan-03-aggregation-engine.md)
**Master plan:** [`master_plan.md`](master_plan.md)

## Goal

A React dashboard that calls the `get_today_summary` Tauri command and renders today's per-app totals, focus-session timeline, and context-switch count. This is the first slice where the app actually looks like Cadence.

## Scope

**In:**
- TypeScript types mirroring the Rust aggregate structs from slice 3
- A typed wrapper around Tauri's `invoke` for `get_today_summary`
- Dashboard components: per-app time breakdown (list or bar chart), focus-session timeline, a context-switch stat card
- Tailwind + shadcn/ui styling (cards, tables/lists — reuse primitives from slice 1)
- Loading and empty states (no data captured yet)

**Out:**
- Historical/multi-day views (post-MVP)
- Start/stop capture controls and error surfacing (slice 5)

## Tasks

- [ ] Define shared TS types for `AppTotal`, `FocusSession`, `TodaySummary` matching the Rust side
- [ ] Build a small `invoke`-wrapping hook, e.g. `useTodaySummary()`, handling loading/error/data states
- [ ] Build `AppTotalsList` (or bar chart) component
- [ ] Build `FocusSessionTimeline` component (simple horizontal timeline is enough for MVP)
- [ ] Build `ContextSwitchStat` component
- [ ] Compose into a single dashboard page/route
- [ ] Handle the "no data yet" empty state with a clear message rather than a blank screen
- [ ] Manual test: capture some real activity (slice 2 loop running), refresh dashboard, confirm numbers match what you actually did
- [ ] Commit as `feat: add dashboard UI for today's focus summary`

## Definition of Done

- Dashboard shows accurate, real data after a capture session, matching manual expectations.
- Empty state renders cleanly with zero captured events.
- No `any` types; components follow [`.cursor/rules/frontend.mdc`](../.cursor/rules/frontend.mdc).

## Next

→ [`plan-05-polish-and-hardening.md`](plan-05-polish-and-hardening.md)
