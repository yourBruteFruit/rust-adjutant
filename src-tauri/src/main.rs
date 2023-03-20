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
async fn fetch_registry(window: tauri::Window) -> String {
    let mut greeting = format!("");
    match fs::read_to_string("data/hero-registry") {
        Result::Ok(data) => {
            greeting = format!("Hello, {}! Welcome to your first day as a hero!", data);
            window.get_window("main").unwrap().show().unwrap();
        }
        Result::Err(_) => {
            window.get_window("registration").unwrap().show().unwrap();
        }
    };
    greeting
}

#[tauri::command]
async fn load_registry(window: tauri::Window) -> String{
    // Close splashscreen
    thread::sleep(time::Duration::from_secs(5));
    if let Some(splashscreen) = window.get_window("splashscreen") {
      splashscreen.close().unwrap();
    }
    fetch_registry(window).await
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            load_registry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
