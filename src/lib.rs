use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::SafeAreaInsetsCss;
#[cfg(mobile)]
use mobile::SafeAreaInsetsCss;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the safe-area-insets-css APIs.
pub trait SafeAreaInsetsCssExt<R: Runtime> {
  fn safe_area_insets_css(&self) -> &SafeAreaInsetsCss<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SafeAreaInsetsCssExt<R> for T {
  fn safe_area_insets_css(&self) -> &SafeAreaInsetsCss<R> {
    self.state::<SafeAreaInsetsCss<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("safe-area-insets-css")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let safe_area_insets_css = mobile::init(app, api)?;
      #[cfg(desktop)]
      let safe_area_insets_css = desktop::init(app, api)?;
      app.manage(safe_area_insets_css);
      Ok(())
    })
    .build()
}
