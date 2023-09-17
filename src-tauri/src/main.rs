// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod model;
mod service;
mod storage;

use api::karma_api::create::create_karma;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
fn set_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    set_tracing();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_karma])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
