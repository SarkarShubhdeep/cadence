use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use active_win_pos_rs::get_active_window;

use crate::db::{Db, WindowEvent};

const POLL_INTERVAL: Duration = Duration::from_secs(1);

/// Identifies which window is focused, independent of how long it's been focused.
#[derive(Debug, Clone, PartialEq, Eq)]
struct WindowIdentity {
    app_name: String,
    window_title: Option<String>,
}

/// A focus span that hasn't ended yet; kept only in memory, never persisted
/// until it closes, so an in-progress span can't corrupt the database.
struct OpenSpan {
    identity: WindowIdentity,
    started_at_ms: i64,
}

pub struct CaptureHandle {
    stop_flag: Arc<AtomicBool>,
    join_handle: JoinHandle<()>,
}

impl CaptureHandle {
    /// Signals the capture loop to stop, waits for it to flush the
    /// in-progress span, and joins the background thread.
    pub fn stop(self) {
        self.stop_flag.store(true, Ordering::SeqCst);
        let _ = self.join_handle.join();
    }
}

pub fn start(db: Arc<Db>) -> CaptureHandle {
    let stop_flag = Arc::new(AtomicBool::new(false));
    let thread_stop_flag = Arc::clone(&stop_flag);

    let join_handle = thread::spawn(move || run_loop(&db, &thread_stop_flag));

    CaptureHandle {
        stop_flag,
        join_handle,
    }
}

fn run_loop(db: &Db, stop_flag: &AtomicBool) {
    let mut open_span: Option<OpenSpan> = None;

    while !stop_flag.load(Ordering::SeqCst) {
        thread::sleep(POLL_INTERVAL);

        if stop_flag.load(Ordering::SeqCst) {
            break;
        }

        if let Some(identity) = snapshot_active_window() {
            open_span = advance_span(db, open_span, identity);
        }
    }

    if let Some(span) = open_span {
        close_span(db, &span, now_ms());
    }
}

fn advance_span(
    db: &Db,
    open_span: Option<OpenSpan>,
    identity: WindowIdentity,
) -> Option<OpenSpan> {
    match open_span {
        Some(span) if span.identity == identity => Some(span),
        Some(span) => {
            close_span(db, &span, now_ms());
            Some(OpenSpan {
                identity,
                started_at_ms: now_ms(),
            })
        }
        None => Some(OpenSpan {
            identity,
            started_at_ms: now_ms(),
        }),
    }
}

fn close_span(db: &Db, span: &OpenSpan, ended_at_ms: i64) {
    let event = WindowEvent {
        app_name: span.identity.app_name.clone(),
        window_title: span.identity.window_title.clone(),
        started_at_ms: span.started_at_ms,
        ended_at_ms,
    };

    if let Err(err) = db.insert_window_event(&event) {
        eprintln!("cadence: failed to persist window event: {err}");
    }
}

fn snapshot_active_window() -> Option<WindowIdentity> {
    match get_active_window() {
        Ok(window) => Some(WindowIdentity {
            app_name: window.app_name,
            window_title: (!window.title.is_empty()).then_some(window.title),
        }),
        Err(()) => {
            eprintln!("cadence: failed to read active window, skipping this poll");
            None
        }
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_millis() as i64)
        .unwrap_or(0)
}
