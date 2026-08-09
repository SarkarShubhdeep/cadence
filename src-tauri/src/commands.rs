use std::sync::{Arc, Mutex};

use chrono::{Duration, Local, NaiveTime, TimeZone};
use tauri::{AppHandle, State};

use crate::aggregate::{self, TodaySummary};
use crate::capture::{self, CaptureHandle};
use crate::db::Db;
use crate::error::CadenceError;

#[derive(Default)]
pub struct CaptureController {
    handle: Mutex<Option<CaptureHandle>>,
}

#[tauri::command]
pub fn start_capture(
    app: AppHandle,
    controller: State<'_, CaptureController>,
    db: State<'_, Arc<Db>>,
) -> Result<(), String> {
    let mut handle = controller
        .handle
        .lock()
        .map_err(|_| "capture controller lock was poisoned".to_string())?;

    if handle.is_none() {
        *handle = Some(capture::start(app, Arc::clone(&db)));
    }

    Ok(())
}

#[tauri::command]
pub fn stop_capture(controller: State<'_, CaptureController>) -> Result<(), String> {
    let mut handle = controller
        .handle
        .lock()
        .map_err(|_| "capture controller lock was poisoned".to_string())?;

    if let Some(running) = handle.take() {
        running.stop();
    }

    Ok(())
}

#[tauri::command]
pub fn is_capturing(controller: State<'_, CaptureController>) -> Result<bool, String> {
    let handle = controller
        .handle
        .lock()
        .map_err(|_| "capture controller lock was poisoned".to_string())?;

    Ok(handle.is_some())
}

#[tauri::command]
pub fn get_today_summary(db: State<'_, Arc<Db>>) -> Result<TodaySummary, String> {
    let (day_start_ms, day_end_ms) = local_day_bounds_ms().map_err(|err| err.to_string())?;
    let events = db
        .window_events_for_day(day_start_ms, day_end_ms)
        .map_err(|err| err.to_string())?;

    Ok(aggregate::summarize(&events))
}

/// Bounds of "today" as `[start, end)` unix ms, using the local calendar day.
fn local_day_bounds_ms() -> Result<(i64, i64), CadenceError> {
    let today = Local::now().date_naive();
    let midnight = today.and_time(NaiveTime::MIN);
    let start_of_day = Local
        .from_local_datetime(&midnight)
        .single()
        .ok_or(CadenceError::AmbiguousLocalTime)?;

    let start_ms = start_of_day.timestamp_millis();
    let end_ms = start_ms + Duration::days(1).num_milliseconds();
    Ok((start_ms, end_ms))
}
