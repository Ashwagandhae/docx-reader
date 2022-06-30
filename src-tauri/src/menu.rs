use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu};

pub fn get_menu() -> Menu {
  return Menu::new()
    .add_submenu(Submenu::new(
      "Docx Reader",
      Menu::new()
        .add_native_item(MenuItem::About(
          "Docx Reader".to_string(),
          // TODO: fix metadata
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
      // TODO make windows compatable
      Menu::new()
        .add_native_item(MenuItem::CloseWindow)
        .add_item(CustomMenuItem::new("open".to_string(), "Open"))
        .add_native_item(MenuItem::Quit),
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
    .add_submenu(Submenu::new("Window", {
      let mut menu = Menu::new();
      menu = menu.add_native_item(MenuItem::Minimize);
      menu = menu.add_native_item(MenuItem::Zoom);
      menu = menu.add_native_item(MenuItem::CloseWindow);
      menu
    }))
    .add_submenu(Submenu::new(
      "Help",
      Menu::new()
        // should open url when clicked:
        .add_item(CustomMenuItem::new("learn more".to_string(), "Learn more")),
    ));
}
