use std::env;

use rusqlite::Connection;

pub fn with_connection(f: impl FnOnce(&Connection) -> Result<(), rusqlite::Error>) {
    let conn = create_connection();
    f(&conn).unwrap();
    conn.close().unwrap();
}

pub fn create_connection() -> Connection {
    let path = env::var("DATABASE_URL").unwrap_or_else(|_| "data.sqlite".to_string());
    Connection::open(path).unwrap()
}
