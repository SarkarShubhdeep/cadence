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

    /// Loads events starting within `[day_start_ms, day_end_ms)`, ordered
    /// chronologically, for the aggregation engine to summarize.
    pub fn window_events_for_day(
        &self,
        day_start_ms: i64,
        day_end_ms: i64,
    ) -> Result<Vec<WindowEvent>, CadenceError> {
        let connection = self
            .connection
            .lock()
            .map_err(|_| CadenceError::LockPoisoned)?;

        let mut statement = connection.prepare(
            "SELECT app_name, window_title, started_at, ended_at \
             FROM window_events \
             WHERE started_at >= ?1 AND started_at < ?2 \
             ORDER BY started_at ASC",
        )?;

        let events = statement
            .query_map(params![day_start_ms, day_end_ms], |row| {
                Ok(WindowEvent {
                    app_name: row.get(0)?,
                    window_title: row.get(1)?,
                    started_at_ms: row.get(2)?,
                    ended_at_ms: row.get(3)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(events)
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
