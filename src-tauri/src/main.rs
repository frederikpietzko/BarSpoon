#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use connection::with_connection;
use dotenv::dotenv;
use migrations::{Migration, MigrationManager};
use std::env;
use tauri::generate_handler;

use crate::todos::{create_todo, delete_todo, get_todos, update_todo};

mod connection;
mod migrations;
mod todos;

fn main() {
    dotenv().ok();
    with_connection(|conn| {
        MigrationManager::new(
            &conn,
            &vec![Migration {
                id: 1,
                name: "Create Todo Table".to_string(),
                applied_at: "2022-08-18".to_string(),
                sql: "CREATE TABLE IF NOT EXISTS T_TODOS (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL,
                descriptions TEXT NOT NULL
            )"
                .to_string(),
            }],
        )
        .migrate()
        .unwrap();
        Ok(())
    });

    tauri::Builder::default()
        .invoke_handler(generate_handler![
            get_todos,
            create_todo,
            delete_todo,
            update_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
