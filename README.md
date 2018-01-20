# R2D2-SQLite

[![Build Status](https://travis-ci.org/gwenn/r2d2-sqlite.svg?branch=master)](https://travis-ci.org/gwenn/r2d2-sqlite)

[rusqlite](https://github.com/jgallagher/rusqlite) support library for the [r2d2](https://github.com/sfackler/r2d2) connection pool.

## Example

```rust
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use std::sync::Arc;
use std::thread;
use std::default::Default;
use r2d2_sqlite::SQLiteConnectionManager;

fn main() {
    let manager = SQLiteConnectionManager::new("sample.db",
                                               rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE);
    let pool = Arc::new(r2d2::Pool::new(manager).unwrap());

    for i in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            conn.execute("INSERT INTO foo (bar) VALUES (?1)", &[&i]).unwrap();
        });
    }
}
```
