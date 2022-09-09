use std::path::PathBuf;
use tauri::api::dialog::blocking::FileDialogBuilder as FileDialogBuilderBlocking;
use tauri::{Runtime, State, Window};

use cocoa::appkit::NSWindowTitleVisibility;
use cocoa::appkit::{NSWindow, NSWindowStyleMask};
use open;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
  pub message: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowCreate {
  pub file_path: String,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct WindowReadyInfo {
  pub file_path: Option<String>,
  pub label: String,
}
pub struct WindowsCreateState {
  pub label_counter: u32,
  pub last_focus: String,
  pub ready: HashMap<String, WindowCreate>,
}
pub struct WindowsCreate(pub Mutex<WindowsCreateState>);

#[tauri::command]
pub fn get_window_fullscreen_state(window: tauri::Window) -> bool {
  window.is_fullscreen().unwrap_or(false)
}
// #[tauri::command]
// pub async fn open_dialog() -> Result<PathBuf, bool> {
//   if let Some(path) = FileDialogBuilderBlocking::new()
//     .add_filter("Word Document", &["docx"])
//     .pick_file()
//   {
//     Ok(path)
//   } else {
//     Err(false)
//   }
// }
#[tauri::command]
pub async fn open_dialog() -> Result<Vec<PathBuf>, bool> {
  if let Some(paths) = FileDialogBuilderBlocking::new()
    .add_filter("Word Document", &["docx"])
    .pick_files()
  {
    Ok(paths)
  } else {
    Err(false)
  }
}
#[tauri::command]
pub fn open_in_word(path: String) {
  match open::that(&path) {
    Ok(()) => println!("opened {} in word", path),
    Err(e) => println!("error opening {} in word: {}", path, e),
  }
}

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
#[tauri::command]
pub fn window_ready(window: Window, windows_create: State<'_, WindowsCreate>) -> WindowReadyInfo {
  window.show().unwrap();
  // if window has creation info
  let windows_create = windows_create.0.lock().unwrap();
  match windows_create.ready.get(window.label()) {
    Some(create) => {
      return WindowReadyInfo {
        file_path: Some(create.file_path.clone()),
        label: window.label().to_string(),
      }
    }
    None => {
      return WindowReadyInfo {
        file_path: None,
        label: window.label().to_string(),
      }
    }
  }
}
#[tauri::command]
pub fn window_focus(window: Window, windows_create: State<WindowsCreate>) {
  let mut windows_create = windows_create.0.lock().unwrap();
  windows_create.last_focus = window.label().to_string();
}

// #[tauri::command]
// pub async fn close_splashscreen(window: tauri::Window) {
//   // Close splashscreen
//   if let Some(splashscreen) = window.get_window("splashscreen") {
//     splashscreen.close().unwrap();
//   }
//   // Show main window
//   window.get_window("main").unwrap().show().unwrap();
// }
