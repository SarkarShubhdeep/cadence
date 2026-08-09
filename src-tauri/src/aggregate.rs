use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::db::WindowEvent;

/// Minimum contiguous time in one app to count as a "focus session" rather
/// than a glance. 30s filters out alt-tab blips and quick checks while still
/// catching short-but-real work blocks; shorter thresholds (e.g. 10s) proved
/// too noisy against normal app-switching habits.
pub const MIN_FOCUS_SESSION_MS: i64 = 30_000;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppTotal {
    pub app_name: String,
    pub total_focused_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocusSession {
    pub app_name: String,
    pub started_at_ms: i64,
    pub ended_at_ms: i64,
    pub duration_ms: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodaySummary {
    pub app_totals: Vec<AppTotal>,
    pub focus_sessions: Vec<FocusSession>,
    pub context_switches: usize,
}

/// Builds the full "today" summary from a day's raw events.
pub fn summarize(events: &[WindowEvent]) -> TodaySummary {
    TodaySummary {
        app_totals: compute_daily_app_totals(events),
        focus_sessions: compute_focus_sessions(events),
        context_switches: compute_context_switches(events),
    }
}

/// Total focused time per app, regardless of session length, sorted with the
/// most-used app first.
pub fn compute_daily_app_totals(events: &[WindowEvent]) -> Vec<AppTotal> {
    let mut totals_by_app: HashMap<String, i64> = HashMap::new();
    for event in events {
        *totals_by_app.entry(event.app_name.clone()).or_insert(0) +=
            event.ended_at_ms - event.started_at_ms;
    }

    let mut app_totals: Vec<AppTotal> = totals_by_app
        .into_iter()
        .map(|(app_name, total_focused_ms)| AppTotal {
            app_name,
            total_focused_ms,
        })
        .collect();
    app_totals.sort_by(|a, b| b.total_focused_ms.cmp(&a.total_focused_ms));
    app_totals
}

/// Coalesces consecutive same-app events (title changes don't break a
/// session) into spans, keeping only those at or above
/// [`MIN_FOCUS_SESSION_MS`].
pub fn compute_focus_sessions(events: &[WindowEvent]) -> Vec<FocusSession> {
    let sorted = sorted_by_start(events);
    let mut sessions = Vec::new();
    let mut current: Option<FocusSession> = None;

    for event in sorted {
        match &mut current {
            Some(session) if session.app_name == event.app_name => {
                session.ended_at_ms = event.ended_at_ms;
                session.duration_ms = session.ended_at_ms - session.started_at_ms;
            }
            _ => {
                if let Some(finished) = current.take() {
                    push_if_long_enough(&mut sessions, finished);
                }
                current = Some(FocusSession {
                    app_name: event.app_name.clone(),
                    started_at_ms: event.started_at_ms,
                    ended_at_ms: event.ended_at_ms,
                    duration_ms: event.ended_at_ms - event.started_at_ms,
                });
            }
        }
    }
    if let Some(finished) = current {
        push_if_long_enough(&mut sessions, finished);
    }

    sessions
}

/// Number of times the active app changed, in chronological order. Title-only
/// changes within the same app don't count.
pub fn compute_context_switches(events: &[WindowEvent]) -> usize {
    let sorted = sorted_by_start(events);
    sorted
        .windows(2)
        .filter(|pair| pair[0].app_name != pair[1].app_name)
        .count()
}

fn sorted_by_start(events: &[WindowEvent]) -> Vec<&WindowEvent> {
    let mut sorted: Vec<&WindowEvent> = events.iter().collect();
    sorted.sort_by_key(|event| event.started_at_ms);
    sorted
}

fn push_if_long_enough(sessions: &mut Vec<FocusSession>, session: FocusSession) {
    if session.duration_ms >= MIN_FOCUS_SESSION_MS {
        sessions.push(session);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(app_name: &str, title: &str, started_at_ms: i64, ended_at_ms: i64) -> WindowEvent {
        WindowEvent {
            app_name: app_name.to_string(),
            window_title: Some(title.to_string()),
            started_at_ms,
            ended_at_ms,
        }
    }

    #[test]
    fn empty_input_yields_zeros() {
        let summary = summarize(&[]);

        assert!(summary.app_totals.is_empty());
        assert!(summary.focus_sessions.is_empty());
        assert_eq!(summary.context_switches, 0);
    }

    #[test]
    fn single_app_all_day_is_one_session_no_switches() {
        let events = vec![event("Code", "main.rs", 0, 3_600_000)];

        let summary = summarize(&events);

        assert_eq!(
            summary.app_totals,
            vec![AppTotal {
                app_name: "Code".to_string(),
                total_focused_ms: 3_600_000,
            }]
        );
        assert_eq!(summary.focus_sessions.len(), 1);
        assert_eq!(summary.focus_sessions[0].duration_ms, 3_600_000);
        assert_eq!(summary.context_switches, 0);
    }

    #[test]
    fn rapid_switching_counts_switches_but_excludes_short_sessions() {
        let events = vec![
            event("Code", "main.rs", 0, 5_000),
            event("Slack", "general", 5_000, 10_000),
            event("Code", "main.rs", 10_000, 15_000),
            event("Slack", "general", 15_000, 20_000),
        ];

        let summary = summarize(&events);

        assert_eq!(summary.context_switches, 3);
        assert!(summary.focus_sessions.is_empty());
        assert_eq!(
            summary
                .app_totals
                .iter()
                .find(|total| total.app_name == "Code")
                .unwrap()
                .total_focused_ms,
            10_000
        );
    }

    #[test]
    fn title_change_within_same_app_merges_session_and_is_not_a_switch() {
        let events = vec![
            event("Code", "main.rs", 0, 20_000),
            event("Code", "lib.rs", 20_000, 40_000),
        ];

        let summary = summarize(&events);

        assert_eq!(summary.context_switches, 0);
        assert_eq!(summary.focus_sessions.len(), 1);
        assert_eq!(summary.focus_sessions[0].started_at_ms, 0);
        assert_eq!(summary.focus_sessions[0].ended_at_ms, 40_000);
        assert_eq!(summary.focus_sessions[0].duration_ms, 40_000);
    }

    #[test]
    fn sessions_below_threshold_are_excluded_but_still_counted_in_totals() {
        let events = vec![
            event("Code", "main.rs", 0, 40_000),
            event("Finder", "Desktop", 40_000, 45_000),
            event("Code", "main.rs", 45_000, 100_000),
        ];

        let summary = summarize(&events);

        assert_eq!(summary.focus_sessions.len(), 2);
        assert!(summary
            .focus_sessions
            .iter()
            .all(|session| session.app_name == "Code"));
        assert_eq!(
            summary
                .app_totals
                .iter()
                .find(|total| total.app_name == "Finder")
                .unwrap()
                .total_focused_ms,
            5_000
        );
        assert_eq!(summary.context_switches, 2);
    }
}
