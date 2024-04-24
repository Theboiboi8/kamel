#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
use tauri::{Manager, Window};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_file_contents(file_path: PathBuf) -> String {
    format!("Getting file contents for file {}", file_path.display())
}

#[tauri::command]
fn get_project(path: PathBuf) -> String {
    path.display().to_string().replace('"', "")
}

#[tauri::command]
fn close_splashscreen(window: Window) {
    window.get_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .close()
        .unwrap();
    window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_file_contents, get_project, close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
