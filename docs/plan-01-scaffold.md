# Slice 1 of 5 — Scaffold & Project Setup

**Status:** Not started
**Depends on:** nothing (first slice)
**Master plan:** [`master_plan.md`](master_plan.md)

## Goal

Get a Tauri v2 + React + TypeScript + Tailwind + shadcn/ui app booting to a window via `npm run tauri dev`, with linting/formatting wired up. No telemetry logic yet — this slice is pure scaffolding.

## Scope

**In:**
- Tauri v2 project structure (Rust `src-tauri/` + frontend root)
- React + TypeScript + Vite frontend
- Tailwind CSS configured
- shadcn/ui installed with a base theme
- ESLint + Prettier for the frontend
- rustfmt + clippy config for the backend
- A placeholder window (title bar, "Cadence" label, empty body)

**Out:**
- Any real capture, storage, or aggregation logic
- Final visual design — this is scaffolding, not the dashboard

## Tasks

- [ ] Scaffold Tauri v2 app (`npm create tauri-app@latest` or manual `src-tauri/` setup) with React + TS template
- [ ] Install and configure Tailwind CSS
- [ ] Install shadcn/ui, run its init, add 2-3 base components (e.g. `card`, `button`) to confirm it works
- [ ] Add `.eslintrc`/`eslint.config.*` + Prettier config matching [`.cursor/rules/frontend.mdc`](../.cursor/rules/frontend.mdc)
- [ ] Add `rustfmt.toml` (if needed) and confirm `cargo clippy -- -D warnings` runs clean on the generated Rust skeleton
- [ ] Confirm `npm run tauri dev` opens a native window with a placeholder "Cadence" screen
- [ ] Update root `README.md` "Getting Started" section with the real, working commands
- [ ] Commit as `feat: scaffold Tauri + React + Tailwind + shadcn app shell`

## Definition of Done

- `npm run tauri dev` opens a window with no errors in either the Rust or JS console.
- `npm run lint` (or equivalent) and `cargo clippy -- -D warnings` both pass on the scaffold.
- README instructions work verbatim on a clean clone.

## Next

→ [`plan-02-capture-and-persistence.md`](plan-02-capture-and-persistence.md)
