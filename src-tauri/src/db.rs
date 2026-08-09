use std::path::Path;
use std::sync::Mutex;

use rusqlite::{params, Connection};

use crate::error::CadenceError;

/// A single completed span of continuous focus on one app/window.
pub struct WindowEvent {
    pub app_name: String,
    pub window_title: Option<String>,
    pub started_at_ms: i64,
    pub ended_at_ms: i64,
}

pub struct Db {
    connection: Mutex<Connection>,
}

impl Db {
    pub fn open(path: &Path) -> Result<Self, CadenceError> {
        let connection = Connection::open(path)?;
        migrate(&connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    /// Persists a completed window event in a single transaction, so a crash
    /// mid-write can never leave a partial row behind.
    pub fn insert_window_event(&self, event: &WindowEvent) -> Result<(), CadenceError> {
        let mut connection = self
            .connection
            .lock()
            .map_err(|_| CadenceError::LockPoisoned)?;

        let tx = connection.transaction()?;
        tx.execute(
            "INSERT INTO window_events (app_name, window_title, started_at, ended_at) \
             VALUES (?1, ?2, ?3, ?4)",
            params![
                event.app_name,
                event.window_title,
                event.started_at_ms,
                event.ended_at_ms
            ],
        )?;
        tx.commit()?;

        Ok(())
    }
}

fn migrate(connection: &Connection) -> Result<(), CadenceError> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS window_events (
            id INTEGER PRIMARY KEY,
            app_name TEXT NOT NULL,
            window_title TEXT,
            started_at INTEGER NOT NULL,
            ended_at INTEGER
        )",
        [],
    )?;

    Ok(())
}
