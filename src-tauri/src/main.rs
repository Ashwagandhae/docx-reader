#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod document;
use document::Document;
use document::Para;

use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::AboutMetadata;
use tauri::State;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct SearchResult {
    link: usize,
    index: usize,
    text: String,
    query_index: usize,
}

struct Paras(Mutex<Vec<Para>>);
struct OutlineParas(Mutex<Vec<Para>>);
struct SearchResultsState {
    results: Vec<SearchResult>,
    last_query: Option<String>,
    para_texts: Vec<String>,
}
struct SearchResults(Mutex<SearchResultsState>);

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
fn load_file(
    path: String,
    paras: State<Paras>,
    outline_paras: State<OutlineParas>,
    search_results: State<SearchResults>,
) -> bool {
    let mut paras = paras.0.lock().unwrap();
    let mut outline_paras = outline_paras.0.lock().unwrap();
    let mut search_results = search_results.0.lock().unwrap();
    // first unload file
    println!("unloading current file");
    paras.clear();
    outline_paras.clear();
    search_results.results.clear();
    search_results.last_query = None;
    search_results.para_texts.clear();
    // then load new one
    println!("loading {:?} file", path);
    let mut doc = Document::new();
    doc.load_file(&path);
    for para in doc.paras {
        let mut combined_text: String = "".to_string();
        for run in para.runs.iter() {
            combined_text.push_str(&run.text);
        }
        search_results.para_texts.push(combined_text);
        paras.push(para);
    }
    for outline_para in doc.outline_paras {
        outline_paras.push(outline_para);
    }
    println!("done loading");
    true
}
#[tauri::command]
fn unload_file(
    paras: State<Paras>,
    outline_paras: State<OutlineParas>,
    search_results: State<SearchResults>,
) -> bool {
    let mut paras = paras.0.lock().unwrap();
    let mut outline_paras = outline_paras.0.lock().unwrap();
    let mut search_results = search_results.0.lock().unwrap();
    paras.clear();
    outline_paras.clear();
    search_results.results.clear();
    search_results.last_query = None;
    search_results.para_texts.clear();
    return true;
}
#[tauri::command]
fn search(
    query: String,
    i: usize,
    j: usize,
    paras: State<'_, Paras>,
    search_results: State<'_, SearchResults>,
) -> Vec<SearchResult> {
    println!("searching with query: {:?}", query);

    let paras = paras.0.lock().unwrap();
    let mut search_results = search_results.0.lock().unwrap();
    let query = query.to_lowercase();
    // decide what to do with search_results.results
    if search_results.last_query.is_some() {
        // if queries are the same, we can keep everything
        if &query == search_results.last_query.as_ref().unwrap() {
            println!("reused old searches")
        // } else
        // // if last query is smaller version of this query, we can narrow down old search results first
        // if query.contains(search_results.last_query.as_ref().unwrap()) {
        //     println!("narrowed down old searches");
        //     // loop through search_results.results and remove all that are not in query
        //     let mut new_results = Vec::new();
        //     for result in search_results.results.iter() {
        //         if result.text.contains(&query) {
        //             new_results.push(result.clone());
        //         }
        //     }
        //     search_results.results = new_results;
        } else
        // if last query is bigger version of this query, we can clear old search results
        {
            println!("cleared old searches");
            search_results.results.clear();
        }
    } else {
        println!("no old searches")
    }
    search_results.last_query = Some(query.clone());
    // fill in the needed search_results.results
    let last_result = search_results.results.last();
    let mut l = 0;
    if last_result.is_some() {
        l = last_result.unwrap().link + 1;
    }
    while search_results.results.len() < j && l < paras.len() {
        let combined_text = search_results.para_texts[l].clone();
        for i in 0..combined_text.to_lowercase().matches(&query).count() {
            let index = search_results.results.len();
            search_results.results.push(SearchResult {
                link: l.clone(),
                index: index,
                text: combined_text.clone(),
                query_index: i,
            });
        }
        l += 1;
    }
    let mut result = Vec::new();
    println!("requested search results: {:?}..{:?}", i, j);
    if search_results.results.len() > 0 {
        for l in i..j {
            if (search_results.results.len() - 1) < l {
                break;
            }
            result.push(search_results.results[l].clone());
        }
    }
    println!("response length: {:?}", result.len());
    result
}
#[tauri::command]
fn clear_search(search_results: State<SearchResults>) -> bool {
    println!("unloading search");
    let mut search_results = search_results.0.lock().unwrap();
    search_results.results.clear();
    search_results.last_query = None;
    return true;
}

use tauri::api::dialog::blocking::FileDialogBuilder as FileDialogBuilderBlocking;
use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
async fn open_dialog() -> Result<PathBuf, bool> {
    if let Some(path) = FileDialogBuilderBlocking::new()
        .add_filter("Word Document", &["docx"])
        .pick_file()
    {
        Ok(path)
    } else {
        Err(false)
    }
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
fn get_outline_paras(i: usize, j: usize, outline_paras: State<OutlineParas>) -> Vec<Para> {
    let outline_paras = outline_paras.0.lock().unwrap();
    let mut result = Vec::new();
    println!("requested outline paragraphs: {:?}..{:?}", i, j);
    if outline_paras.len() > 0 {
        for l in i..j {
            if (outline_paras.len() - 1) < l {
                break;
            }
            result.push(outline_paras[l].clone());
        }
    }
    println!("response length: {:?}", result.len());
    return result;
}

fn main() {
    let open_menu_item = CustomMenuItem::new("open".to_string(), "Open");

    let menu = Menu::new()
        .add_submenu(Submenu::new(
            "Docx Reader",
            Menu::new()
                .add_native_item(MenuItem::About(
                    "Docx Reader".to_string(),
                    // todo: fix metadata
                    AboutMetadata::new()
                        .version("0.1.0")
                        .website("https://github.com/Ashwagandhae/docx-reader")
                        .authors(vec!["Ashwagandhae".to_string()]),
                ))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ))
        .add_submenu(Submenu::new(
            "File",
            // todo make windows compatable
            Menu::new()
                .add_native_item(MenuItem::CloseWindow)
                .add_item(open_menu_item), // Menu::new().add_native_item(MenuItem::Quit)
        ))
        .add_submenu(Submenu::new("Edit", {
            let mut menu = Menu::new();
            menu = menu.add_native_item(MenuItem::Undo);
            menu = menu.add_native_item(MenuItem::Redo);
            menu = menu.add_native_item(MenuItem::Separator);
            menu = menu.add_native_item(MenuItem::Cut);
            menu = menu.add_native_item(MenuItem::Copy);
            menu = menu.add_native_item(MenuItem::Paste);
            #[cfg(not(target_os = "macos"))]
            {
                menu = menu.add_native_item(MenuItem::Separator);
            }
            menu = menu.add_native_item(MenuItem::SelectAll);
            menu
        }))
        .add_submenu(Submenu::new(
            "View",
            Menu::new().add_native_item(MenuItem::EnterFullScreen),
        ))
        .add_submenu(Submenu::new(
            "Window",
            Menu::new()
                .add_native_item(MenuItem::Minimize)
                .add_native_item(MenuItem::Zoom)
                // todo make windows compatable
                .add_native_item(MenuItem::Separator),
        ));
    // todo add help
    // .add_submenu(Submenu::new(
    //     "Help",
    //     Menu::new()
    //         // should open url when clicked:
    //         .add_item(MenuItem::Url("Learn More", url)),
    // ));

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "open" => FileDialogBuilder::new()
                .add_filter("Word Document", &["docx"])
                .pick_file(move |path| {
                    if path.is_some() {
                        event
                            .window()
                            .emit(
                                "load_file",
                                Payload {
                                    message: path.unwrap().to_string_lossy().into_owned(),
                                },
                            )
                            .unwrap()
                    }
                }),
            _ => {}
        })
        .manage(Paras(Mutex::new(Vec::new())))
        .manage(OutlineParas(Mutex::new(Vec::new())))
        .manage(SearchResults(Mutex::new(SearchResultsState {
            results: Vec::new(),
            last_query: None,
            para_texts: Vec::new(),
        })))
        .invoke_handler(tauri::generate_handler![
            load_file,
            get_paras,
            search,
            clear_search,
            unload_file,
            get_outline_paras,
            open_dialog
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
