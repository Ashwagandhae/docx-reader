#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{Runtime, Window};
mod document;

mod app_commands;
mod document_commands;

use document_commands::{OutlineParas, Paras, SearchResults, SearchResultsState};

use std::sync::Mutex;

use tauri::AboutMetadata;
use tauri::Manager;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool) {
        use cocoa::appkit::NSWindowTitleVisibility;

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
        .setup(|app| {
            let win = app.get_window("main").unwrap();
            win.set_transparent_titlebar(true);

            Ok(())
        })
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
            document_commands::load_file,
            document_commands::get_paras,
            document_commands::search,
            document_commands::clear_search,
            document_commands::unload_file,
            document_commands::get_outline_paras,
            app_commands::open_dialog,
            app_commands::get_window_fullscreen_state,
            app_commands::new_window,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
