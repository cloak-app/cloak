use crate::store::model::AppStoreKey;
use crate::store::{reset_app_store, set_to_app_store};
use crate::utils::shortcut::AppShortcut;
use crate::utils::window::{close_reader_window, open_reader_window};
use serde_json::Value;
use tauri::{Emitter, Manager};
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
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn get_all_font_families() -> Result<Vec<String>, String> {
    let source = font_kit::source::SystemSource::new();
    let font_list = source.all_families().map_err(|e| e.to_string())?;
    Ok(font_list)
}

#[tauri::command]
pub fn set_font_family(app_handle: tauri::AppHandle, font_family: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontFamily, &font_family)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_line_height(app_handle: tauri::AppHandle, line_height: f64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::LineHeight, &line_height)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_letter_spacing(app_handle: tauri::AppHandle, letter_spacing: i64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::LetterSpacing, &letter_spacing)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_font_weight(app_handle: tauri::AppHandle, font_weight: i64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontWeight, &font_weight)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_font_color(app_handle: tauri::AppHandle, font_color: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontColor, &font_color)?;
    app_handle.emit("config-change", 0).unwrap();
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

#[tauri::command]
pub fn unset_shortcut(app_handle: tauri::AppHandle, shortcut: String) -> Result<(), String> {
    let shortcut = shortcut.parse::<Shortcut>().map_err(|e| e.to_string())?;

    AppShortcut::unregister_shortcut(&app_handle, shortcut)?;

    Ok(())
}

#[tauri::command]
pub fn reset_config(app_handle: tauri::AppHandle) -> Result<(), String> {
    reset_app_store(&app_handle)?;
    Ok(())
}
