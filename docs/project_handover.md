# Project Handover: Developer Telemetry Dashboard

---

## Project Overview

This project is a purely local telemetry application designed to monitor, aggregate, and visualize development workflows. It captures coding habits, keystroke frequency, active window time, and commit patterns to help developers identify peak productivity hours and manage context switching. By running entirely on the host machine without external network dependencies, it ensures complete data privacy and an exceptionally low system memory footprint.

## Technical Architecture

| Component           | Technology                                 | Purpose                                                                                                                                           |
| ------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Desktop Runtime** | Tauri v2                                   | Cross-platform desktop application packaging and system-level API access. It leverages the OS's native web renderer to keep binary sizes minimal. |
| **Frontend UI**     | React, TypeScript, Tailwind CSS, shadcn/ui | Provides a fast, type-safe, and highly customizable minimalist user interface.                                                                    |
| **Local Backend**   | Rust                                       | High-performance, memory-safe local backend for capturing and processing rapid telemetry data directly from the OS.                               |
| **Local Storage**   | SQLite                                     | Lightweight, file-based relational database managed by the Rust backend to securely persist all telemetry data directly on the user's local disk. |

---

## Core Features

- **Activity Logging:** Silently tracks active window duration, IDE usage, and specific file types being edited.
- **Privacy-First Aggregation:** Calculates keystroke frequency and activity levels without logging raw key inputs or sensitive content.
- **Context Visualization:** Maps out focus periods and context-switching rates throughout the work day to highlight workflow efficiency.
- **Data Sovereignty:** All activity metrics and historical data remain 100% on the local device, requiring no internet connection or third-party cloud accounts.

---

## Development Setup Instructions

1. Clone the repository to your local machine.
2. Ensure you have the necessary system prerequisites installed for Tauri development, including Rust and Node.js.
3. Run `npm install` in the root directory to install all frontend dependencies.
4. Review the `.cursor/rules` file to familiarize yourself with the repository's strict formatting and codebase standards before writing new code.
5. Run `npm run tauri dev` to launch the frontend development server alongside the Rust compilation process, which enables hot-reloading for UI changes.

---

## Known Limitations & Next Steps

- **Data Pruning:** Because all historical data is stored locally, a scheduled job or user setting needs to be implemented to archive or compress older SQLite records to prevent the database file from growing indefinitely.
- **System API Variances:** While Tauri v2 is cross-platform, system-level activity tracking behaves differently on macOS, Windows, and Linux. Deeper testing and custom Rust adapters are required for robust support across all operating systems.
- **Data Portability:** Without a cloud sync mechanism, the application requires an export/import utility (e.g., CSV or JSON) to allow users to back up their data or migrate it to a new machine.
