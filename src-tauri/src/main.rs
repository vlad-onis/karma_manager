// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod model;
mod service;
mod storage;

use model::karma::{KarmaPoint, KarmaType};
use storage::db::DbManager;
use tracing::info;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use storage::karma_repository::KarmaRepository;

fn set_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    set_tracing();
    let db = DbManager::new("test.sqlite").await.unwrap();
    let karma = KarmaPoint::new(KarmaType::Sleeping);
    let karma = db.insert_karma(karma).await;
    info!("{karma:?}");
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
