// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod permissions;
mod processes;

use std::vec;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_admin_permissions() -> bool {
    permissions::elevated()
}

#[tauri::command]
fn get_process_list() -> Vec<String> {
   processes::list_all_processes()
}

fn main() {
    println!("Is Admin: {}", permissions::elevated());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_admin_permissions, get_process_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
