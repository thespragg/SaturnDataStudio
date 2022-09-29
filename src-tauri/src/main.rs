#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod db;
use db::db::PostgresConnection;

use crate::db::db::generate_schema;

#[tokio::main]
async fn main() {
    generate_schema();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_connection, get_connections])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn create_connection(
    name: &str,
    conn: &str,
    username: &str,
    password: &str,
) -> Result<bool, bool> {
    return match db::db::insert_connection(&name, &conn, &username, &password).await {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    };
}

#[tauri::command]
async fn get_connections() -> Result<Vec<PostgresConnection>, String> {
    match db::db::get_connections().await {
        Ok(vals) => Ok(vals),
        Err(_) => Err("Something went wrong getting the connections".to_string()),
    }
}
