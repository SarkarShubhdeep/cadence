use std::sync::{Arc, Mutex};

use tauri::State;

use crate::capture::{self, CaptureHandle};
use crate::db::Db;

#[derive(Default)]
pub struct CaptureController {
    handle: Mutex<Option<CaptureHandle>>,
}

#[tauri::command]
pub fn start_capture(
    controller: State<'_, CaptureController>,
    db: State<'_, Arc<Db>>,
) -> Result<(), String> {
    let mut handle = controller
        .handle
        .lock()
        .map_err(|_| "capture controller lock was poisoned".to_string())?;

    if handle.is_none() {
        *handle = Some(capture::start(Arc::clone(&db)));
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
