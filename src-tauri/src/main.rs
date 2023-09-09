// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use win_screenshot::prelude::*;
use image::RgbaImage;
use windows::Win32::UI::WindowsAndMessaging::{WindowFromPoint, GetCursorPos, GetWindowRect, GetWindowTextW};
use windows::Win32::Foundation::{HWND, POINT};


fn get_mouse_pos() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut point) };
    (point.x, point.y)
}

fn get_window_id(x: i32, y: i32) -> isize {
    let mut point = POINT { x, y };
    let window = unsafe { WindowFromPoint(point) };
    window.0 as isize
}

fn get_window_text(window_id: isize) -> String {
    let window = HWND(window_id as isize);
    let mut text = String::new();
    unsafe {
        let mut buffer: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, &mut buffer);
        if len > 0 {
            text = String::from_utf16_lossy(&buffer[..len as usize]);
        }
    }
    text
}

#[tauri::command]
fn get_window() -> (isize, String) {
    let (x, y) = get_mouse_pos();
    let window_id = get_window_id(x, y);
    let window_text = get_window_text(window_id);
    (window_id, window_text)
}

#[tauri::command]
fn screenshot(window_id: isize) -> () {
    let using = Using::PrintWindow;
    //todo make faster
    //https://stackoverflow.com/questions/43595289/screenshot-with-bitblt-results-in-black-image-on-windows-10
    //let using = Using::BitBlt;
    let area = Area::ClientOnly;
    let crop_xy = None;
    let crop_wh = None;
    let buf = capture_window_ex(window_id, using, area, crop_xy, crop_wh).unwrap();

    // convert to image and save
    let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    img.save("../screenshot.jpg").unwrap();
    println!("Saved screenshot.jpg");
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|e| {
            if let tauri::WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .invoke_handler(tauri::generate_handler![screenshot, get_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
