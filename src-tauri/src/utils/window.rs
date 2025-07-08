use crate::store::get_from_app_store;
use crate::store::model::AppStoreKey;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub fn destroy_reader_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("reader");

    if let Some(window) = window {
        window.close().unwrap();
    }

    Ok(())
}

pub fn open_reader_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("reader");

    if let Some(window) = window {
        window.set_focus().unwrap();
    } else {
        let always_on_top =
            get_from_app_store::<bool>(app_handle, AppStoreKey::AlwaysOnTop).unwrap();

        let transparent = get_from_app_store::<bool>(app_handle, AppStoreKey::Transparent).unwrap();

        WebviewWindowBuilder::new(app_handle, "reader", WebviewUrl::default())
            .min_inner_size(200.0, 50.0)
            .shadow(false)
            .transparent(transparent)
            .decorations(false)
            .always_on_top(always_on_top)
            .build()
            .unwrap();
    }

    Ok(())
}

pub fn open_settings_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("settings");

    if let Some(window) = window {
        window.set_focus().unwrap();
    } else {
        WebviewWindowBuilder::new(app_handle, "settings", WebviewUrl::default())
            .min_inner_size(800.0, 600.0)
            .build()
            .unwrap();
    }

    Ok(())
}

pub fn open_update_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle.get_webview_window("update");

    if let Some(window) = window {
        window.set_focus().unwrap();
    } else {
        WebviewWindowBuilder::new(app_handle, "update", WebviewUrl::default())
            .min_inner_size(800.0, 600.0)
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
