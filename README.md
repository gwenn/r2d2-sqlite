# R2D2-SQLite

[![Build Status](https://travis-ci.org/gwenn/r2d2-sqlite.svg?branch=master)](https://travis-ci.org/gwenn/r2d2-sqlite)

[rusqlite](https://github.com/jgallagher/rusqlite) support library for the [r2d2](https://github.com/sfackler/r2d2) connection pool.

**BEWARE:** _Not_ related to [r2d2_sqlite](https://crates.io/crates/r2d2_sqlite) nor [r2d2-sqlite3](https://crates.io/crates/r2d2-sqlite3) crates ([Jul 5, 2015](https://github.com/gwenn/r2d2-sqlite/commit/551d1f71523653441acda6e1e6ee800edd492c86) vs [Jul 22, 2015](https://github.com/ivanceras/r2d2-sqlite/commit/8ea9f422493bb2fb7b4eea71e6a82a35d2dfca9c)).

## Example

```rust

use std::sync::Arc;
use std::thread;
use std::default::Default;
use r2d2::Pool;
use r2d2_sqlite::SQLiteConnectionManager;
use rusqlite::OpenFlags;

fn main() {
    let manager = SQLiteConnectionManager::new("sample.db",
                                               OpenFlags::SQLITE_OPEN_READ_WRITE);
    let pool = Arc::new(Pool::new(manager).unwrap());

    for i in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            conn.execute("INSERT INTO foo (bar) VALUES (?1)", &[&i]).unwrap();
        });
    }
}
```
