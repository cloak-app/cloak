use crate::commands::shortcut::AppShortcut;
use crate::state::AppStoreKey;
use crate::utils::*;
use serde_json::Value;
use tauri::Manager;
use tauri_plugin_global_shortcut::Shortcut;
use tauri_plugin_store::StoreExt;

/* ---------------------------------- 基础设置 ---------------------------------- */

#[tauri::command]
pub fn get_config(app_handle: tauri::AppHandle) -> Result<Value, String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    // 获取所有键值对
    let all_entries = store.entries();

    let mut obj = serde_json::Map::new();
    for (key, value) in all_entries {
        obj.insert(key, value);
    }
    let config = Value::Object(obj);

    Ok(config)
}

#[tauri::command]
pub fn set_dock_visibility(
    app_handle: tauri::AppHandle,
    dock_visibility: bool,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::DockVisibility, &dock_visibility)?;

    #[cfg(target_os = "macos")]
    app_handle
        .set_dock_visibility(dock_visibility)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn set_always_on_top(app_handle: tauri::AppHandle, always_on_top: bool) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::AlwaysOnTop, &always_on_top)?;

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
    set_to_app_store(&app_handle, AppStoreKey::Transparent, &transparent)?;

    if force_reopen {
        close_reader_window(&app_handle).unwrap();
        open_reader_window(&app_handle).unwrap();
    }

    Ok(())
}

/* ---------------------------------- 阅读设置 ---------------------------------- */

#[tauri::command]
pub fn set_font_size(app_handle: tauri::AppHandle, font_size: i64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontSize, &font_size)?;
    Ok(())
}

#[tauri::command]
pub fn set_font_family(app_handle: tauri::AppHandle, font_family: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontFamily, &font_family)?;
    Ok(())
}

#[tauri::command]
pub fn set_line_height(app_handle: tauri::AppHandle, line_height: i64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::LineHeight, &line_height)?;
    Ok(())
}

#[tauri::command]
pub fn set_font_weight(app_handle: tauri::AppHandle, font_weight: i64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontWeight, &font_weight)?;
    Ok(())
}

#[tauri::command]
pub fn set_font_color(app_handle: tauri::AppHandle, font_color: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontColor, &font_color)?;
    Ok(())
}

#[tauri::command]
pub fn set_background_color(
    app_handle: tauri::AppHandle,
    background_color: String,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::BackgroundColor, &background_color)?;
    Ok(())
}

/* ---------------------------------- 快捷键设置 --------------------------------- */

#[tauri::command]
pub fn set_next_line_shortcut(
    app_handle: tauri::AppHandle,
    shortcut: String,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::NextLineShortcut, &shortcut)?;

    let shortcut = shortcut.parse::<Shortcut>().map_err(|e| e.to_string())?;

    AppShortcut::register_shortcut(&app_handle, AppShortcut::NextLine(shortcut))?;

    Ok(())
}

#[tauri::command]
pub fn set_prev_line_shortcut(
    app_handle: tauri::AppHandle,
    shortcut: String,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::PrevLineShortcut, &shortcut)?;

    let shortcut = shortcut.parse::<Shortcut>().map_err(|e| e.to_string())?;

    AppShortcut::register_shortcut(&app_handle, AppShortcut::PrevLine(shortcut))?;

    Ok(())
}

#[tauri::command]
pub fn set_boss_key_shortcut(app_handle: tauri::AppHandle, shortcut: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::BossKeyShortcut, &shortcut)?;

    let shortcut = shortcut.parse::<Shortcut>().map_err(|e| e.to_string())?;

    AppShortcut::register_shortcut(&app_handle, AppShortcut::BossKey(shortcut))?;

    Ok(())
}
