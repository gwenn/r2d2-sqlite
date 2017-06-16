extern crate rusqlite;
extern crate r2d2;
extern crate r2d2_sqlite;

use std::sync::mpsc;
use std::thread;

use r2d2_sqlite::SQLiteConnectionManager;
use rusqlite::{SQLITE_OPEN_URI, SQLITE_OPEN_CREATE, SQLITE_OPEN_READ_WRITE};

#[test]
fn test_basic() {
    let manager = SQLiteConnectionManager::new(
        "file:dummy.db?mode=memory&cache=shared",
        SQLITE_OPEN_URI | SQLITE_OPEN_CREATE | SQLITE_OPEN_READ_WRITE,
    );
    let config = r2d2::Config::builder().pool_size(2).build();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    let (s1, r1) = mpsc::channel();
    let (s2, r2) = mpsc::channel();

    let pool1 = pool.clone();
    let t1 = thread::spawn(move || {
        let conn = pool1.get().unwrap();
        s1.send(()).unwrap();
        r2.recv().unwrap();
        drop(conn);
    });

    let pool2 = pool.clone();
    let t2 = thread::spawn(move || {
        let conn = pool2.get().unwrap();
        s2.send(()).unwrap();
        r1.recv().unwrap();
        drop(conn);
    });

    t1.join().unwrap();
    t2.join().unwrap();

    pool.get().unwrap();
}

#[test]
fn test_is_valid() {
    let manager = SQLiteConnectionManager::new(
        "file:dummy.db?mode=memory&cache=shared",
        SQLITE_OPEN_URI | SQLITE_OPEN_CREATE | SQLITE_OPEN_READ_WRITE,
    );
    let config = r2d2::Config::builder()
        .pool_size(1)
        .test_on_check_out(true)
        .build();
    let pool = r2d2::Pool::new(config, manager).unwrap();

    pool.get().unwrap();
}
