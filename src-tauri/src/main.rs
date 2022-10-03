#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod http;
pub mod store;
pub mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![http::client::get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
