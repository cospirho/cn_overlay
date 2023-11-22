// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dictionary;
use dictionary::{parse_dictionary, DictionaryEntry};

use win_screenshot::prelude::*;
use image::RgbaImage;
use windows::Win32::UI::WindowsAndMessaging::{WindowFromPoint, GetCursorPos, GetWindowRect, GetWindowTextW, GetDesktopWindow};
use windows::Win32::Foundation::{HWND, POINT};
use reqwest::blocking::Client;
use std::fs::File;
use std::collections::HashMap;

struct DictionaryInstance(HashMap<String, DictionaryEntry>);

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
fn get_window() -> (isize, String, i32, i32, i32, i32) {
    let (x, y) = get_mouse_pos();
    let window_id = get_window_id(x, y);
    let window_text = get_window_text(window_id);
    let window_rect = unsafe {
        let mut rect = std::mem::zeroed();
        GetWindowRect(HWND(window_id as isize), &mut rect);
        rect
    };
    (window_id, window_text, window_rect.left, window_rect.top, window_rect.right, window_rect.bottom)
}

//ocr server is running on port 7272
fn get_ocr_data() -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open("../screenshot.jpg")?;

    let client = Client::new();
    let res = client.post("http://localhost:7272/ocr")
        .header("Content-Type", "multipart/form-data")
        .body(file)
        .send()?;
    let text = res.text()?;
    Ok(text)
}

#[tauri::command]
//crop_wh is a Option<[i32; 2]>
fn screenshot(window_id: isize, crop_wh: Option<[i32; 2]>, crop_xy: Option<[i32; 2]>) -> String {
    let using = Using::PrintWindow;
    //TODO make faster? We want the window since the overlay will cover the actual text
    //https://stackoverflow.com/questions/43595289/screenshot-with-bitblt-results-in-black-image-on-windows-10
    //let using = Using::BitBlt;
    println!("crop_wh: {:?}", crop_wh);
    let area = Area::ClientOnly;
    //crop_wh is the width and height OF the screenshot
    //crop_xy is the x and y position of the screenshot
    let buf = capture_window_ex(window_id, using, area, crop_xy, crop_wh).unwrap();

    // convert to image and save
    let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    
    //for debugging, we also save the screenshot
    println!("Saving screenshot...");
    img.save("../screenshot.jpg").unwrap();
    println!("Saved screenshot.jpg");
    let ocr_result = get_ocr_data();
    println!("OCR result: {:?}", ocr_result);
    //handle error
    let ocr_result = match ocr_result {
        Ok(text) => text,
        Err(e) => format!("Error: {}", e)
    };
    ocr_result
}

// fuck it TODO make more efficient
pub fn substrings(s: &str) -> Vec<String> {
    //todo what chars are actually possible?
    //let word_ending = vec!['!', '?', '。', '！', '？', '，', '、', ' ', '.', ',', '_'];
    let mut result = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let maxlen = 4;

    for i in 0..chars.len() {
        let mut subsequence = String::new();
        for j in i..i + maxlen {
            if j < chars.len() {
                subsequence.push(chars[j]);
                result.push(subsequence.clone());
            }
        }
    }
    result
}


pub fn get_definitions(dictionary: &HashMap<String, DictionaryEntry>, sentence:&str) -> Vec<(usize, String, String, String)> {
    let substrings = substrings(sentence);
    let not_found = "Not found".to_string();
    let mut found_words = Vec::new();
    // lookup each substring in the dictionary
    for (i, substring) in substrings.iter().enumerate() {
        let result = dictionary.get(substring);
        let (pinyin, definitions) = match result {
            Some(entry) => (&entry.pinyin, &entry.definitions),
            None => {
                (&not_found, &not_found)
            }
        };
        if definitions != &not_found {
            found_words.push((i, pinyin.clone(), definitions.clone(), substring.clone()));
        }
    } 
    found_words
}

#[tauri::command]
fn lookup_sentence(state: tauri::State<DictionaryInstance>, sentence:&str) -> Vec<(usize, String, String, String)> {
    let found_words = get_definitions(&state.0, sentence);
    found_words
}

// users can highlight a word to look it up
#[tauri::command]
fn lookup_word(state: tauri::State<DictionaryInstance>, word:&str) -> (String, String) {
    let not_found = "Not found".to_string();
    let result = &state.0.get(word);
        let (pinyin, definitions) = match result {
            Some(entry) => (&entry.pinyin, &entry.definitions),
            None => {
                (&not_found, &not_found)
            }
        };
    (pinyin.clone(), definitions.clone())
}

fn main() {
    println!("Parsing dictionary...");
    let dictionary = parse_dictionary();
    println!("Dictionary parsed");

    tauri::Builder::default()
        .manage(DictionaryInstance(dictionary))
        .on_window_event(|e| {
            if let tauri::WindowEvent::Resized(_) = e.event() {
                std::thread::sleep(std::time::Duration::from_nanos(1));
            }
        })
        .invoke_handler(tauri::generate_handler![screenshot, get_window, lookup_sentence, lookup_word])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
