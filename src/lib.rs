//! SQLite support for the `r2d2` connection pool.
#![warn(missing_docs)]
extern crate r2d2;
extern crate rusqlite;

use std::path::PathBuf;

/// An `r2d2::ManageConnection` for `rusqlite::SqliteConnection`s.
///
/// ## Example
///
/// ```rust
/// extern crate r2d2;
/// extern crate r2d2_sqlite;
/// extern crate rusqlite;
///
/// use std::path::PathBuf;
/// use std::sync::Arc;
/// use std::default::Default;
/// use std::thread;
/// use r2d2_sqlite::SQLiteConnectionManager;
/// use rusqlite::{SQLITE_OPEN_URI, SQLITE_OPEN_CREATE, SQLITE_OPEN_READ_WRITE};
///
/// fn main() {
///     let config = r2d2::Config::default();
///     let manager = SQLiteConnectionManager::new(PathBuf::from("file:dummy.db?mode=memory&cache=shared"),
///             SQLITE_OPEN_URI | SQLITE_OPEN_CREATE | SQLITE_OPEN_READ_WRITE);
///     let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());
///
///     for i in 0..10i32 {
///         let pool = pool.clone();
///         thread::spawn(move || {
///             let conn = pool.get().unwrap();
///             conn.execute("PRAGMA user_version=?1", &[&i]).unwrap();
///         });
///     }
/// }
/// ```
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
