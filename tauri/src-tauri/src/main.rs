//! Forge Tauri Application Entry Point
//!
//! This is the main entry point for the Forge desktop application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use forge_app::{create_shared_state, SharedState};
use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer().json())
        .with(EnvFilter::from_default_env().add_directive("forge=info".parse().unwrap()))
        .init();

    info!("Starting Forge v{}", env!("CARGO_PKG_VERSION"));

    // Create shared application state
    let state = create_shared_state();

    // Build and run Tauri application
    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            info!("Forge application setup complete");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // TODO: Register IPC command handlers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
