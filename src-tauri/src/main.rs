#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod binary;

pub use binary::{ Binary, Section };

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // commands::install_patches
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}