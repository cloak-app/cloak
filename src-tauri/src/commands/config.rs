use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn set_dock_visibility<R: Runtime>(app: AppHandle<R>, visible: bool) -> Result<(), String> {
    let store = app.store("app_data.json").unwrap();

    store.set("dock_visibility", visible);

    #[cfg(target_os = "macos")]
    app.set_dock_visibility(visible).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_always_on_top<R: Runtime>(app: AppHandle<R>, always_on_top: bool) -> Result<(), String> {
    let store = app.store("app_data.json").unwrap();

    store.set("always_on_top", always_on_top);

    let window = app.get_webview_window("reader");

    if let Some(window) = window {
        window.set_always_on_top(always_on_top).unwrap();
    }

    Ok(())
}
