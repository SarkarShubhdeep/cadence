# Slice 5 of 5 — Polish & Hardening

**Status:** Not started
**Depends on:** [`plan-04-dashboard-ui.md`](plan-04-dashboard-ui.md)
**Master plan:** [`master_plan.md`](master_plan.md)

## Goal

Close the loop into something demo-able and stable: manual controls, error handling, a clean lint pass, and the MVP definition-of-done from the master plan actually verified end-to-end.

## Scope

**In:**
- Manual start/stop capture control surfaced in the UI (using the Tauri commands from slice 2)
- Error handling/surfacing for capture failures and DB errors (toast/banner, not a silent console log)
- Full clippy + ESLint clean pass across the whole codebase
- Basic app branding (window title, icon) if not already set in slice 1
- README updated with real run instructions and a screenshot
- A tagged `v0.1.0-mvp` commit/release

**Out:**
- Anything from the post-MVP roadmap (keystrokes, commit tracking, history, export/import, cross-platform)

## Tasks

- [ ] Add a start/stop toggle in the UI wired to the slice-2 Tauri commands
- [ ] Surface capture/DB errors to the user instead of failing silently
- [ ] Run `cargo clippy -- -D warnings` and the frontend linter across the full repo; fix everything
- [ ] Set app icon + window title if still placeholder from slice 1
- [ ] Verify against every bullet in [`master_plan.md` §10 Success Criteria](master_plan.md#10-success-criteria-definition-of-done-for-mvp)
- [ ] Take a screenshot of the working dashboard, add to README
- [ ] Tag the commit/release as `v0.1.0-mvp`

## Definition of Done

- All items in the master plan's "Definition of Done for MVP" are verifiably true.
- No network activity occurs at any point during a full run (spot-check with a network monitor).
- Fresh clone → README steps → working app, with no undocumented manual steps.

## Next

MVP complete. Revisit [`master_plan.md` §11 Post-MVP Roadmap](master_plan.md#11-post-mvp-roadmap) to plan the next set of slices.
