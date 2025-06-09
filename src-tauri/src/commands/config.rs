use crate::window::*;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub fn set_dock_visibility(app_handle: tauri::AppHandle, visible: bool) -> Result<(), String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    store.set("dock_visibility", visible);

    #[cfg(target_os = "macos")]
    app_handle
        .set_dock_visibility(visible)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn set_always_on_top(app_handle: tauri::AppHandle, always_on_top: bool) -> Result<(), String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    store.set("always_on_top", always_on_top);

    let window = app_handle.get_webview_window("reader");

    if let Some(window) = window {
        window
            .set_always_on_top(always_on_top)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn set_transparent(
    app_handle: tauri::AppHandle,
    transparent: bool,
    force_reopen: bool,
) -> Result<(), String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    store.set("transparent", transparent);

    if force_reopen {
        close_reader_window(&app_handle).unwrap();
        open_reader_window(&app_handle).unwrap();
    }

    Ok(())
}
