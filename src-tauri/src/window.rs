use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;

pub fn open_reader_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("reader");

    if let Some(window) = window {
        window.set_focus().unwrap();
    } else {
        let store = app_handle.get_store("app_data.json").unwrap();
        let always_on_top = store
            .get("always_on_top")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let transparent = store
            .get("transparent")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        WebviewWindowBuilder::new(app_handle, "reader", WebviewUrl::default())
            .shadow(false)
            .transparent(transparent)
            .always_on_top(always_on_top)
            .inner_size(200.0, 100.0)
            .build()
            .unwrap();
    }

    Ok(())
}

pub fn close_reader_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("reader");

    if let Some(window) = window {
        window.destroy().unwrap();
    }

    Ok(())
}

pub fn open_settings_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("settings");

    if let Some(window) = window {
        window.set_focus().unwrap();
    } else {
        WebviewWindowBuilder::new(app_handle, "settings", WebviewUrl::default())
            .inner_size(800.0, 600.0)
            .build()
            .unwrap();
    }

    Ok(())
}
