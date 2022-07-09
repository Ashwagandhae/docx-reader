#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::api::dialog::FileDialogBuilder;
use tauri::{AppHandle, Runtime, State, Window, WindowBuilder, WindowUrl};

mod document;

mod app_commands;
mod document_commands;
mod menu;

use app_commands::{Payload, WindowCreate, WindowsCreate, WindowsCreateState};
use document_commands::{OutlineParas, Paras, SearchResults, SearchResultsState};

use menu::get_menu;

use std::sync::Mutex;

use cocoa::appkit::NSWindowTitleVisibility;
use tauri::api::shell::open;
use tauri::Manager;

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool) {
        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;

            let mut style_mask = id.styleMask();
            style_mask.set(
                NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                transparent,
            );
            id.setStyleMask_(style_mask);

            id.setTitleVisibility_(if transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });
            id.setTitlebarAppearsTransparent_(if transparent {
                cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}
// TODO remember which window to focus on when opening multiple
#[tauri::command]
fn new_window(
    creates: Vec<Option<WindowCreate>>,
    app: AppHandle,
    window: Window,
    paras: State<'_, Paras>,
    outline_paras: State<'_, OutlineParas>,
    search_results: State<'_, SearchResults>,
    windows_create: State<'_, WindowsCreate>,
) -> Result<(), ()> {
    let url = WindowUrl::App(PathBuf::from("index.html"));
    let mut windows_create = windows_create.0.lock().unwrap();
    let mut paras_dict = paras.0.lock().unwrap();
    let mut outline_paras_dict = outline_paras.0.lock().unwrap();
    let mut search_results_dict = search_results.0.lock().unwrap();

    // get window from last focus, to allow for recently created windows
    // default to window that emitted command
    let window = app.get_window(&windows_create.last_focus).unwrap_or(window);

    let mut pos = window.inner_position().unwrap();
    let size = window.inner_size().unwrap();
    for create in creates {
        let label = "normal".to_string() + &windows_create.label.to_string();
        windows_create.label += 1;

        paras_dict.insert(label.clone(), Vec::new());
        outline_paras_dict.insert(label.clone(), Vec::new());
        search_results_dict.insert(
            label.clone(),
            SearchResultsState {
                results: Vec::new(),
                last_query: None,
                para_texts: Vec::new(),
            },
        );

        pos.x += 80;
        pos.y += 80;
        // using as will break if width/height > 2147483647
        // ... it won't
        if (pos.x + size.width as i32)
            > window.current_monitor().unwrap().unwrap().size().width as i32
        {
            pos.x -= size.width as i32;
        }
        if (pos.y + size.height as i32)
            > window.current_monitor().unwrap().unwrap().size().height as i32
        {
            pos.y -= size.height as i32;
        }
        println!("creating new window, pos: {:?}", pos);
        let new_window = WindowBuilder::new(&app, label.clone(), url.clone())
            .title("Docx Reader")
            .visible(false)
            .position((pos.x / 2).into(), (pos.y / 2).into())
            .inner_size((size.width / 2).into(), (size.height / 2).into())
            .min_inner_size(300.0, 200.0)
            .build()
            .unwrap();
        new_window.hide().unwrap();
        if create.is_some() {
            // set creation info
            windows_create.ready.insert(label.clone(), create.unwrap());
        }
        // set last focus for quickly created windows
        windows_create.last_focus = label;
    }
    #[cfg(target_os = "macos")]
    {
        // win.set_transparent_titlebar(true);
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            win.hide().unwrap();
            #[cfg(target_os = "macos")]
            {
                // win.set_transparent_titlebar(true);
            }
            app.listen_global("tauri://focus", |event| {
                println!("{:?}", event.payload());
            });

            Ok(())
        })
        .menu(get_menu())
        .on_menu_event(|event| match event.menu_item_id() {
            "open" => FileDialogBuilder::new()
                .add_filter("Word Document", &["docx"])
                .pick_file(move |path| {
                    if path.is_some() {
                        println!("opening file in window: {:?}", event.window().label());
                        event
                            .window()
                            .emit_to(
                                event.window().label(),
                                "load_file",
                                Payload {
                                    message: path.unwrap().to_string_lossy().into_owned(),
                                },
                            )
                            .unwrap()
                    }
                }),
            "learn more" => {
                match open(
                    &(event.window()).shell_scope(),
                    "https://github.com/Ashwagandhae/docx-reader",
                    None,
                ) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Couldn't open URL: {}", e);
                    }
                }
            }
            _ => {}
        })
        .manage(Paras(Mutex::new(HashMap::from([(
            "main".to_string(),
            Vec::new(),
        )]))))
        .manage(OutlineParas(Mutex::new(HashMap::from([(
            "main".to_string(),
            Vec::new(),
        )]))))
        .manage(WindowsCreate(Mutex::new(WindowsCreateState {
            label: 0,
            last_focus: "main".into(),
            ready: HashMap::new(),
        })))
        .manage(SearchResults(Mutex::new(HashMap::from([(
            "main".to_string(),
            SearchResultsState {
                results: Vec::new(),
                last_query: None,
                para_texts: Vec::new(),
            },
        )]))))
        .invoke_handler(tauri::generate_handler![
            document_commands::load_file,
            document_commands::get_paras,
            document_commands::search,
            document_commands::clear_search,
            document_commands::unload_file,
            document_commands::get_outline_paras,
            document_commands::get_nearest_outline_para,
            app_commands::open_dialog,
            app_commands::get_window_fullscreen_state,
            app_commands::window_ready,
            app_commands::window_focus,
            new_window,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
