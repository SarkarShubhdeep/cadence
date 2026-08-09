mod aggregate;
mod capture;
mod commands;
mod db;
mod error;

use std::sync::Arc;

use tauri::Manager;

use commands::CaptureController;
use db::Db;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let db = Db::open(&app_data_dir.join("cadence.db"))?;
            app.manage(Arc::new(db));
            app.manage(CaptureController::default());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_capture,
            commands::stop_capture,
            commands::is_capturing,
            commands::get_today_summary,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
