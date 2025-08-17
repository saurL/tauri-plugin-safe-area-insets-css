use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<SafeAreaInsetsCss<R>> {
  Ok(SafeAreaInsetsCss(app.clone()))
}

/// Access to the safe-area-insets-css APIs.
pub struct SafeAreaInsetsCss<R: Runtime>(AppHandle<R>);

impl<R: Runtime> SafeAreaInsetsCss<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
