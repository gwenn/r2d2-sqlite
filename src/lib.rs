//! SQLite support for the `r2d2` connection pool.
#![warn(missing_docs)]

use std::convert::Into;
use std::path::PathBuf;

/// An `r2d2::ManageConnection` for `rusqlite::SqliteConnection`s.
///
/// ## Example
///
/// ```rust
/// use r2d2_sqlite::SQLiteConnectionManager;
/// use rusqlite::OpenFlags;
/// use std::default::Default;
/// use std::sync::Arc;
/// use std::thread;
///
///     let manager = SQLiteConnectionManager::new(
///         "file:dummy.db?mode=memory&cache=shared",
///         OpenFlags::SQLITE_OPEN_URI
///             | OpenFlags::SQLITE_OPEN_CREATE
///             | OpenFlags::SQLITE_OPEN_READ_WRITE,
///     );
///     let pool = Arc::new(r2d2::Pool::new(manager).unwrap());
///
///     for i in 0..10i32 {
///         let pool = pool.clone();
///         thread::spawn(move || {
///             let conn = pool.get().unwrap();
///             conn.execute("PRAGMA user_version=?1", &[&i]).unwrap();
///         });
///     }
/// ```
pub struct SQLiteConnectionManager {
    path: PathBuf,
    flags: rusqlite::OpenFlags,
}

impl SQLiteConnectionManager {
    /// Creates a new `SQLiteConnectionManager`.
    ///
    /// See `rusqlite::SqliteConnection::open_with_flags` for a description of
    /// the parameter types.
    pub fn new<T>(path: T, flags: rusqlite::OpenFlags) -> SQLiteConnectionManager
    where
        T: Into<PathBuf>,
    {
        SQLiteConnectionManager {
            path: path.into(),
            flags,
        }
    }
}

impl r2d2::ManageConnection for SQLiteConnectionManager {
    type Connection = rusqlite::Connection;
    type Error = rusqlite::Error;

    fn connect(&self) -> Result<rusqlite::Connection, rusqlite::Error> {
        rusqlite::Connection::open_with_flags(&self.path, self.flags)
    }

    fn is_valid(&self, conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
        // http://sqlite.org/pragma.html#pragma_schema_version
        conn.execute_batch("PRAGMA schema_version")
    }

    fn has_broken(&self, conn: &mut rusqlite::Connection) -> bool {
        // pending transaction or at least one statement busy
        !conn.is_autocommit() || conn.is_busy()
    }
}

// http://sqlite.org/c3ref/db_release_memory.html
// http://sqlite.org/c3ref/interrupt.html
