/** Mirrors `AppTotal` in `src-tauri/src/aggregate.rs`. */
export interface AppTotal {
  appName: string;
  totalFocusedMs: number;
}

/** Mirrors `FocusSession` in `src-tauri/src/aggregate.rs`. */
export interface FocusSession {
  appName: string;
  startedAtMs: number;
  endedAtMs: number;
  durationMs: number;
}

/** Mirrors `TodaySummary` in `src-tauri/src/aggregate.rs`. */
export interface TodaySummary {
  appTotals: AppTotal[];
  focusSessions: FocusSession[];
  contextSwitches: number;
}
