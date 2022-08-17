#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dotenv::dotenv;
use migrations::{Migration, MigrationManager};
use rusqlite::Connection;
use std::env;
use todos::TodoRepository;

mod migrations;
mod todos;

fn main() {
    dotenv().ok();
    let path = env::var("DATABASE_URL").unwrap_or_else(|_| "data.sqlite".to_string());
    let conn = Connection::open(path).unwrap();

    MigrationManager::new(
        &conn,
        &vec![Migration {
            id: 1,
            name: "Create Todo Table".to_string(),
            applied_at: "2022-08-18".to_string(),
            sql: "CREATE TABLE IF NOT EXISTS T_TODOS (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL,
                descriptions TEXT NOT NULL
            )"
            .to_string(),
        }],
    )
    .migrate()
    .unwrap();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    conn.close().unwrap();
}
