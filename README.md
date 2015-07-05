# R2D2-SQLite
[rusqlite](https://github.com/jgallagher/rusqlite) support library for the [r2d2](https://github.com/sfackler/r2d2) connection pool.

## Example

```rust
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::default::Default;
use r2d2_sqlite::SQLiteConnectionManager;

fn main() {
    let config = r2d2::Config::default();
    let manager = SQLiteConnectionManager::new(PathBuf::from("sample.db"),
                                               rusqlite::SQLITE_OPEN_READ_WRITE);
    let pool = Arc::new(r2d2::Pool::new(config, manager).unwrap());

    for i in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
            conn.execute("INSERT INTO foo (bar) VALUES (?1)", &[&i]).unwrap();
        });
    }
}
```