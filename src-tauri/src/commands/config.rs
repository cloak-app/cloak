use crate::db::Db;
use crate::state::model::AppState;
use crate::store::model::AppStoreKey;
use crate::store::{get_entries_from_app_store, reset_app_store, set_to_app_store};
use crate::utils::reader::NovelReader;
use crate::utils::shortcut;
use crate::utils::sql;
use serde_json::Value;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;

/* ---------------------------------- 偏好设置 ---------------------------------- */
#[tauri::command]
pub fn get_last_check_update_time(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Option<i64>, String> {
    let state = state.lock().map_err(|e| e.to_string())?;

    Ok(state.update_checker.last_check_time)
}

#[tauri::command]
pub fn set_check_update_interval(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
    interval: u64,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;

    let update_checker = &mut state.update_checker;

    set_to_app_store(&app_handle, AppStoreKey::CheckUpdateInterval, interval)?;

    update_checker
        .start(&app_handle, interval)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn set_auto_start(app_handle: tauri::AppHandle, auto_start: bool) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::AutoStart, auto_start)?;

    if auto_start {
        app_handle
            .autolaunch()
            .enable()
            .map_err(|e| e.to_string())?;
    } else {
        app_handle
            .autolaunch()
            .disable()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn set_language(app_handle: tauri::AppHandle, language: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::Language, language)?;
    Ok(())
}

#[tauri::command]
pub fn set_theme(app_handle: tauri::AppHandle, theme: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::Theme, theme)?;
    Ok(())
}

/* ---------------------------------- 基础设置 ---------------------------------- */
#[tauri::command]
pub fn set_dock_visibility(
    app_handle: tauri::AppHandle,
    dock_visibility: bool,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::DockVisibility, dock_visibility)?;

    #[cfg(target_os = "macos")]
    app_handle
        .set_dock_visibility(dock_visibility)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn set_always_on_top(app_handle: tauri::AppHandle, always_on_top: bool) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::AlwaysOnTop, always_on_top)?;

    let window = app_handle.get_webview_window("reader");

    if let Some(window) = window {
        window
            .set_always_on_top(always_on_top)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub fn set_transparent(app_handle: tauri::AppHandle, transparent: bool) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::Transparent, transparent)?;
    Ok(())
}

/* ---------------------------------- 阅读设置 ---------------------------------- */

#[tauri::command]
pub async fn set_line_size(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    line_size: usize,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::LineSize, line_size)?;

    let (novel_id, read_position, read_progress) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let Some(reader) = &mut state.novel_reader else {
            return Err("暂无打开的小说".to_string());
        };

        let current_chapter = reader.current_chapter();

        let (lines, chapters) = NovelReader::read_lines(&reader.novel_path, line_size)?;

        let new_read_position = chapters
            .iter()
            .find(|chapter| chapter.index == current_chapter.index)
            .map(|chapter| chapter.start_line)
            .expect("当前章节丢失");

        reader.lines = lines;
        reader.chapters = chapters;
        reader.read_position = new_read_position;
        (
            reader.novel_id,
            reader.read_position as i64,
            reader.read_progress(),
        )
    };

    sql::save_novel(&db, novel_id, read_position, read_progress).await?;

    app_handle.emit("reader-change", 0).unwrap();

    Ok(())
}

#[tauri::command]
pub fn set_font_size(app_handle: tauri::AppHandle, font_size: i64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontSize, font_size)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_font_family(app_handle: tauri::AppHandle, font_family: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontFamily, font_family)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_line_height(app_handle: tauri::AppHandle, line_height: f64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::LineHeight, line_height)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_letter_spacing(app_handle: tauri::AppHandle, letter_spacing: f64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::LetterSpacing, letter_spacing)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_font_weight(app_handle: tauri::AppHandle, font_weight: i64) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontWeight, font_weight)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

#[tauri::command]
pub fn set_font_color(app_handle: tauri::AppHandle, font_color: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::FontColor, font_color)?;
    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}

/* ---------------------------------- 快捷键设置 --------------------------------- */

#[tauri::command]
pub fn set_next_line_shortcut(
    app_handle: tauri::AppHandle,
    shortcut: String,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::NextLineShortcut, shortcut)?;
    Ok(())
}

#[tauri::command]
pub fn set_prev_line_shortcut(
    app_handle: tauri::AppHandle,
    shortcut: String,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::PrevLineShortcut, shortcut)?;
    Ok(())
}

#[tauri::command]
pub fn set_next_chapter_shortcut(
    app_handle: tauri::AppHandle,
    shortcut: String,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::NextChapterShortcut, shortcut)?;
    Ok(())
}

#[tauri::command]
pub fn set_prev_chapter_shortcut(
    app_handle: tauri::AppHandle,
    shortcut: String,
) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::PrevChapterShortcut, shortcut)?;
    Ok(())
}

#[tauri::command]
pub fn set_boss_key_shortcut(app_handle: tauri::AppHandle, shortcut: String) -> Result<(), String> {
    set_to_app_store(&app_handle, AppStoreKey::BossKeyShortcut, shortcut)?;
    Ok(())
}

#[tauri::command]
pub fn set_toggle_reading_mode_shortcut(
    app_handle: tauri::AppHandle,
    shortcut: String,
) -> Result<(), String> {
    set_to_app_store(
        &app_handle,
        AppStoreKey::ToggleReadingModeShortcut,
        shortcut,
    )?;
    Ok(())
}

#[tauri::command]
pub fn unregister_all_shortcuts(app_handle: tauri::AppHandle) -> Result<(), String> {
    shortcut::unregister_all_shortcuts(&app_handle)?;
    Ok(())
}

#[tauri::command]
pub fn activate_all_shortcuts(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let reading_mode = state.reading_mode;

    let shortcuts = if reading_mode {
        shortcut::AppShortcut::all_shortcuts()
    } else {
        shortcut::AppShortcut::common_shortcuts()
    };

    shortcut::activate_shortcuts(&app_handle, shortcuts)?;

    Ok(())
}

/* ----------------------------------- 其他 ----------------------------------- */
#[tauri::command]
pub fn get_config(app_handle: tauri::AppHandle) -> Result<Value, String> {
    let all_entries = get_entries_from_app_store(&app_handle)?;

    let mut obj = serde_json::Map::new();
    for (key, value) in all_entries {
        obj.insert(key, value);
    }
    let config = Value::Object(obj);

    Ok(config)
}

#[tauri::command]
pub fn reset_config(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    reset_app_store(&app_handle)?;

    shortcut::unregister_all_shortcuts(&app_handle)?;

    let state = state.lock().map_err(|e| e.to_string())?;
    let reading_mode = state.reading_mode;

    let shortcuts = if reading_mode {
        shortcut::AppShortcut::all_shortcuts()
    } else {
        shortcut::AppShortcut::common_shortcuts()
    };

    shortcut::activate_shortcuts(&app_handle, shortcuts)?;

    app_handle.emit("config-change", 0).unwrap();
    Ok(())
}
