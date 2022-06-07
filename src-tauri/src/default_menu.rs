impl Menu {
  pub fn default() -> Menu {
    Menu::new()
      .add_default_submenu(Submenu::App)
      .add_default_submenu(Submenu::File)
      .add_default_submenu(Submenu::Edit)
      .add_default_submenu(Submenu::View)
      .add_default_submenu(Submenu::Window)
      .add_default_submenu(Submenu::Help("https://tauri.studio"))
  }

  pub fn add_default_submenu(&mut self, submenu: Submenu) {
    match submenu {
      Submenu::App => {
        #[cfg(target_os = "macos")]
        self.add_submenu(Submenu::new(
          app_name,
          Menu::new()
            .add_native_item(MenuItem::About(app_name))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Services)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Hide)
            .add_native_item(MenuItem::HideOthers)
            .add_native_item(MenuItem::ShowAll)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
        ));
      }

      Submenu::File => {
        self.add_submenu(Submenu::new(
          "File",
          if cfg!(target_os == "macos") {
            Menu::new().add_native_item(MenuItem::CloseWindow)
          } else {
            Menu::new().add_native_item(MenuItem::Quit)
          },
        ));
      }

      Submenu::Edit => {
        self.add_submenu(Submenu::new("Edit", {
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
        }));
      }

      Submenu::View => {
        self.add_submenu(Submenu::new(
          "View",
          Menu::new().add_native_item(MenuItem::EnterFullScreen),
        ));
      }

      Submenu::Window => {
        self.add_submenu(Submenu::new("Window", {
          let menu = Menu::new()
            .add_native_item(MenuItem::Minimize)
            .add_native_item(MenuItem::Zoom);
          if cfg!(target_os == "macos") {
            menu.add_native_item(MenuItem::Separator);
            // not yet supported:
            menu.add_native_item(MenuItem::BringAllToFront);
            // not yet supported (window selector):
            menu.add_native_item(MenuItem::Window);
          } else {
            menu.add_native_item(MenuItem::CloseWindow);
          }
        }));
      }

      Submenu::Help(url) => {
        self.add_submenu(Submenu::new(
          "Help",
          Menu::new()
            // should open url when clicked:
            .add_item(MenuItem::Url("Learn More", url)),
        ))
      }
    }
  }
}
