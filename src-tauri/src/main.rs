#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#[tauri::command]
async fn drag_window(window: tauri::Window) {
    window.start_dragging().unwrap();
}
#[tauri::command]
async fn minimize_window(window: tauri::Window) {
    window.minimize().unwrap();
}
#[tauri::command]
async fn close_window(window: tauri::Window) {
    window.close().unwrap();
}
#[tauri::command]
async fn maximize_window(window: tauri::Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let _ = window.set_title("VolleyGroups");
            let _ = window.set_focus();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            drag_window,
            minimize_window,
            close_window,
            maximize_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
