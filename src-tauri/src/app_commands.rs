use std::path::PathBuf;
use tauri::api::dialog::blocking::FileDialogBuilder as FileDialogBuilderBlocking;
use tauri::{AppHandle, Manager, Runtime, Window, WindowBuilder, WindowUrl};

use cocoa::appkit::NSWindowTitleVisibility;
use cocoa::appkit::{NSWindow, NSWindowStyleMask};

#[tauri::command]
pub fn get_window_fullscreen_state(window: tauri::Window) -> bool {
  window.is_fullscreen().unwrap_or(false)
}
#[tauri::command]
pub async fn open_dialog() -> Result<PathBuf, bool> {
  if let Some(path) = FileDialogBuilderBlocking::new()
    .add_filter("Word Document", &["docx"])
    .pick_file()
  {
    Ok(path)
  } else {
    Err(false)
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
pub async fn new_window(window: Window) -> Result<(), ()> {
  let url = WindowUrl::App(PathBuf::from("index.html"));

  println!("create new window: {}", url);

  let win = WindowBuilder::new(&window, "test", url)
    .title("hola")
    .build()
    .unwrap();
  win.set_transparent_titlebar(true);
  Ok(())
}
