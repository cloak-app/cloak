pub mod model;

use crate::store::model::AppStoreKey;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const DEFAULT_NEXT_LINE_SHORTCUT: &str = "Control+Alt+ArrowRight";
const DEFAULT_PREV_LINE_SHORTCUT: &str = "Control+Alt+ArrowLeft";
const DEFAULT_NEXT_CHAPTER_SHORTCUT: &str = "Control+Alt+ArrowDown";
const DEFAULT_PREV_CHAPTER_SHORTCUT: &str = "Control+Alt+ArrowUp";
const DEFAULT_BOSS_KEY_SHORTCUT: &str = "Control+Alt+Enter";

pub fn reset_app_store(app_handle: &AppHandle) -> Result<(), String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    store.set(AppStoreKey::DockVisibility.as_str(), false);
    store.set(AppStoreKey::AlwaysOnTop.as_str(), false);
    store.set(AppStoreKey::Transparent.as_str(), true);
    store.set(AppStoreKey::LineSize.as_str(), 50);
    store.set(AppStoreKey::FontSize.as_str(), 16);
    store.set(AppStoreKey::LineHeight.as_str(), 1.5);
    store.set(AppStoreKey::LetterSpacing.as_str(), 0);
    store.set(AppStoreKey::FontWeight.as_str(), 400);
    store.set(AppStoreKey::FontColor.as_str(), "#000000");
    store.set(
        AppStoreKey::NextLineShortcut.as_str(),
        DEFAULT_NEXT_LINE_SHORTCUT,
    );
    store.set(
        AppStoreKey::PrevLineShortcut.as_str(),
        DEFAULT_PREV_LINE_SHORTCUT,
    );
    store.set(
        AppStoreKey::NextChapterShortcut.as_str(),
        DEFAULT_NEXT_CHAPTER_SHORTCUT,
    );
    store.set(
        AppStoreKey::PrevChapterShortcut.as_str(),
        DEFAULT_PREV_CHAPTER_SHORTCUT,
    );
    store.set(
        AppStoreKey::BossKeyShortcut.as_str(),
        DEFAULT_BOSS_KEY_SHORTCUT,
    );

    store.save().unwrap();

    Ok(())
}

pub fn init_app_store(app_handle: &AppHandle) -> Result<(), String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    if store.is_empty() {
        reset_app_store(app_handle)?;
    }

    Ok(())
}

pub fn get_from_app_store<T: serde::de::DeserializeOwned>(
    app_handle: &AppHandle,
    key: AppStoreKey,
) -> Result<Option<T>, String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    let value = store
        .get(key.as_str())
        .map(|v| serde_json::from_value(v.clone()))
        .transpose()
        .map_err(|e| e.to_string())?;

    Ok(value)
}

pub fn set_to_app_store<T: serde::Serialize>(
    app_handle: &AppHandle,
    key: AppStoreKey,
    value: T,
) -> Result<(), String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    let json_value = serde_json::to_value(value).map_err(|e| e.to_string())?;
    store.set(key.as_str(), json_value);
    store.save().unwrap();

    Ok(())
}
