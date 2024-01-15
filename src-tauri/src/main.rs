// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::{Manager, Window};

// default tauri greet command on main window.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



// set shadow on window splashscreen
#[tauri::command]
fn set_shadow(window: Window) {
	window.get_window("splashscreen").expect("no window found");
	#[cfg(any(windows, target_os = "macos"))]
	window_shadows::set_shadow(&window, true).unwrap();
}


// command closes the splashscreen and then opens the main window
#[tauri::command]
async fn close_splashscreen(window: Window) {
  // close splashscreen
  window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
  // show main window
  window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, close_splashscreen, set_shadow])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
