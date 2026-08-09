# Cadence

A local-first developer telemetry dashboard. Cadence quietly observes your coding rhythm - active window time, keystroke cadence, and commit patterns - and turns it into a private, on-device view of your focus and context-switching over the day.

![status](https://img.shields.io/badge/status-early%20development-yellow)
![license](https://img.shields.io/badge/license-MIT-blue)
![platform](https://img.shields.io/badge/platform-desktop-lightgrey)

## Why

Most productivity trackers ship your activity data to a server. Cadence doesn't. Everything is captured, aggregated, and stored on your own machine - no accounts, no network calls, no cloud dependency.

## Tech Stack

- **Desktop shell:** Tauri v2 - packages the app cross-platform using the OS's native web renderer, keeping the binary small.
- **Frontend:** React, TypeScript, Tailwind CSS, shadcn/ui - a fast, type-safe, minimalist UI.
- **Backend:** Rust - memory-safe, high-performance capture and processing of telemetry from the OS.
- **Storage:** SQLite - a single local file, managed entirely by the Rust backend.

## Core Features

- **Activity logging** - tracks active window duration, IDE usage, and file types being edited.
- **Privacy-first aggregation** - computes keystroke frequency and activity levels without ever storing raw keystrokes or file contents.
- **Context visualization** - surfaces focus periods and context-switching rates across the work day.
- **Data sovereignty** - all data stays on your device; nothing leaves without your explicit export.

## Status

This is a weekend project in early development. The app shell boots, a Rust background loop captures active-window/app changes into a local SQLite file (start/stop from the app window), and a `get_today_summary` command now aggregates those events into per-app totals, focus sessions, and a context-switch count for today; the dashboard comes next. See [Roadmap](#roadmap) below.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain) and platform build tools required by [Tauri v2](https://v2.tauri.app/start/prerequisites/)
- [Node.js](https://nodejs.org/) 18+
- To capture window titles (not just app names), grant the app Screen Recording permission in System Settings → Privacy & Security

## Getting Started

```bash
git clone https://github.com/SarkarShubhdeep/cadence.git
cd cadence
npm install
npm run tauri dev
```

Review [`.cursor/rules/`](.cursor/rules/) before contributing - it captures this repo's coding standards and engineering principles.

## Roadmap

- [x] Scaffold the Tauri v2 + React + TypeScript + Tailwind app shell
- [ ] Rust backend: OS-level activity capture (active window, keystroke cadence)
- [ ] SQLite schema for aggregated telemetry
- [ ] Dashboard UI for focus/context-switching visualization
- [ ] Data pruning/archiving for long-running local databases
- [ ] Cross-platform testing (macOS, Windows, Linux) for system API differences
- [ ] Export/import (CSV or JSON) for backup and machine migration

See [`docs/project_handover.md`](docs/project_handover.md) for the original technical handover.

## License

[MIT](LICENSE) (c) 2026 Shubhdeep Sarkar
