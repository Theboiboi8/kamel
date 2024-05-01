#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use tauri::{Manager, Window};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn pick_file() -> (PathBuf, String) {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Open File:")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)
        .expect("Failed to get file");

    load_file(handle.path().to_owned()).await.expect("Failed to read file contents")
}

async fn load_file(path : PathBuf) -> Result<(PathBuf, String), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await.unwrap_or(String::new());


    Ok((path, contents))
}

#[tauri::command]
fn get_project(path: PathBuf) {
    
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
        .invoke_handler(tauri::generate_handler![pick_file, get_project, close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
    IOFailed(std::io::ErrorKind)
}