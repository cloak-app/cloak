use crate::utils::state::AppStoreKey;
use crate::utils::store::get_from_app_store;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub fn open_reader_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("reader");

    if let Some(window) = window {
        window.set_focus().unwrap();
    } else {
        let always_on_top =
            get_from_app_store::<bool>(app_handle, AppStoreKey::AlwaysOnTop)?.unwrap_or(false);

        let transparent =
            get_from_app_store::<bool>(app_handle, AppStoreKey::Transparent)?.unwrap_or(true);

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

pub fn hide_all_windows(app_handle: &AppHandle) -> Result<(), String> {
    let windows = app_handle.webview_windows();

    for (_, window) in windows {
        window.hide().unwrap();
    }

    Ok(())
}

pub fn show_all_windows(app_handle: &AppHandle) -> Result<(), String> {
    let windows = app_handle.webview_windows();

    for (_, window) in windows {
        window.show().unwrap();
    }

    Ok(())
}
