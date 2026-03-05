use rusqlite::Connection;
use tracing::debug;

use crate::sensors::Reading;

pub struct SqliteBuffer {
    conn: Connection,
}

impl SqliteBuffer {
    pub fn new(db_path: &str) -> anyhow::Result<Self> {
        let conn = Connection::open(db_path)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS readings_buffer (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                payload TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            CREATE INDEX IF NOT EXISTS idx_readings_buffer_created_at
                ON readings_buffer(created_at);",
        )?;

        Ok(Self { conn })
    }

    pub fn push(&self, readings: &[Reading]) -> anyhow::Result<()> {
        let payload = serde_json::to_string(readings)?;
        self.conn.execute(
            "INSERT INTO readings_buffer (payload) VALUES (?1)",
            [&payload],
        )?;
        debug!("Buffered {} readings", readings.len());
        Ok(())
    }

    pub fn pop(&self, limit: usize) -> anyhow::Result<Vec<(i64, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, payload FROM readings_buffer ORDER BY id ASC LIMIT ?1",
        )?;

        let rows = stmt
            .query_map([limit as i64], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    pub fn delete(&self, ids: &[i64]) -> anyhow::Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "DELETE FROM readings_buffer WHERE id IN ({})",
            placeholders.join(",")
        );

        let params: Vec<Box<dyn rusqlite::types::ToSql>> =
            ids.iter().map(|id| Box::new(*id) as Box<dyn rusqlite::types::ToSql>).collect();
        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        self.conn.execute(&sql, param_refs.as_slice())?;
        debug!("Deleted {} buffered entries", ids.len());
        Ok(())
    }

    pub fn count(&self) -> anyhow::Result<usize> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM readings_buffer", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}
