//! SQLite support for the `r2d2` connection pool.
extern crate r2d2;
extern crate rusqlite;

use std::path::PathBuf;

pub struct SQLiteConnectionManager {
    path: PathBuf,
    flags: rusqlite::SqliteOpenFlags
}

impl SQLiteConnectionManager {
    /// Creates a new `SQLiteConnectionManager`.
    ///
    /// See `rusqlite::SqliteConnection::open_with_flags` for a description of
    /// the parameter types.
    pub fn new(path: PathBuf, flags: rusqlite::SqliteOpenFlags)
            -> SQLiteConnectionManager {
        SQLiteConnectionManager {
            path: path,
            flags: flags,
        }
    }
}

impl r2d2::ManageConnection for SQLiteConnectionManager {
    type Connection = rusqlite::SqliteConnection;
    type Error = rusqlite::SqliteError;

    fn connect(&self) -> Result<rusqlite::SqliteConnection, rusqlite::SqliteError> {
        rusqlite::SqliteConnection::open_with_flags(&self.path, self.flags)
    }

    fn is_valid(&self, conn: &mut rusqlite::SqliteConnection) -> Result<(), rusqlite::SqliteError> {
        // http://sqlite.org/pragma.html#pragma_schema_version
        conn.execute_batch("PRAGMA schema_verion")
    }

    fn has_broken(&self, conn: &mut rusqlite::SqliteConnection) -> bool {
        if !conn.is_autocommit() { // pending transaction
            true
        } else if conn.is_busy() { // at least one statement busy
            true
        } else {
            false
        }
    }
}

// http://sqlite.org/c3ref/db_release_memory.html
// http://sqlite.org/c3ref/interrupt.html
