// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, thread, time};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    fs::write("data/hero-registry", name).expect("Unable to write file");
    format!("Hello, {}! Welcome to your first day as a hero!", name)
}

#[tauri::command]
async fn load_registry() -> String {
    let data= fs::read_to_string("data/hero-registry").expect("Unable to read file");
    format!("Hello, {}! Welcome to your first day as a hero!", data)
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    thread::sleep(time::Duration::from_secs(5));
    if let Some(splashscreen) = window.get_window("splashscreen") {
      splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_registry,
            close_splashscreen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
