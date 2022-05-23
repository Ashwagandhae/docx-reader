#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod document;
use document::Document;
use document::Drop;
use document::OutlineItem;
use document::Para;

use std::sync::Mutex;

use tauri::State;

struct Paras(Mutex<Vec<Para>>);
struct OutlineItems(Mutex<Vec<OutlineItem>>);

#[tauri::command]
fn load_file(filepath: String, paras: State<Paras>, outline_items: State<OutlineItems>) -> bool {
    let mut doc = Document::new();
    doc.load_file(&filepath);
    let mut paras = paras.0.lock().unwrap();
    for para in doc.paras {
        paras.push(para);
    }
    let mut outline_items = outline_items.0.lock().unwrap();
    for outline_item in doc.outline_items {
        outline_items.push(outline_item);
    }
    return true;
}

#[tauri::command]
fn unload_file(paras: State<Paras>, outline_items: State<OutlineItems>) -> bool {
    let mut paras = paras.0.lock().unwrap();
    paras.clear();
    let mut outline_items = outline_items.0.lock().unwrap();
    outline_items.clear();
    return true;
}

#[tauri::command]
fn get_paras(i: usize, j: usize, paras: State<Paras>) -> Vec<Para> {
    let paras = paras.0.lock().unwrap();
    let mut result = Vec::new();
    println!("requested paragraphs: {:?}..{:?}", i, j);
    if paras.len() > 0 {
        for l in i..j {
            if (paras.len() - 1) < l {
                break;
            }
            result.push(paras[l].clone());
        }
    }
    println!("response length: {:?}", result.len());
    return result;
}

#[tauri::command]
fn get_outline_items(i: usize, j: usize, outline_items: State<OutlineItems>) -> Vec<OutlineItem> {
    let outline_items = outline_items.0.lock().unwrap();
    let mut result = Vec::new();
    println!("requested outline paragraphs: {:?}..{:?}", i, j);
    if outline_items.len() > 0 {
        for l in i..j {
            if (outline_items.len() - 1) < l {
                break;
            }
            result.push(outline_items[l].clone());
        }
    }
    println!("response length: {:?}", result.len());
    return result;
}

fn main() {
    tauri::Builder::default()
        .manage(Paras(Mutex::new(Vec::new())))
        .manage(OutlineItems(Mutex::new(Vec::new())))
        .invoke_handler(tauri::generate_handler![
            load_file,
            get_paras,
            unload_file,
            get_outline_items
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
