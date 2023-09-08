// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {

    let docs_window = tauri::WindowBuilder::new(
        &handle,
        "external", /* the unique window label */
        tauri::WindowUrl::App("overlay.html".into())
    ).transparent(true)
    .inner_size(1000.0, 1000.0)
    .resizable(true)
    .decorations(false)
    .build().unwrap();

    //.decorations(false)
    //docs_window.set_ignore_cursor_events(true);
    docs_window.set_always_on_top(true);

    // create transparent window
}
fn main() {
    tauri::Builder::default()
        .on_window_event(|e| {
            if let tauri::WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .invoke_handler(tauri::generate_handler![greet,  open_docs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
