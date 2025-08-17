use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::SafeAreaInsetsCssExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.safe_area_insets_css().ping(payload)
}
