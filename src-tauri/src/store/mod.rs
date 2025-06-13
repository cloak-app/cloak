pub mod model;

use crate::commands::config::{
    set_boss_key_shortcut, set_next_line_shortcut, set_prev_line_shortcut,
};
use crate::store::model::AppStoreKey;
use crate::utils::shortcut::AppShortcut;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const DEFAULT_NEXT_LINE_SHORTCUT: &str = "Control+Alt+ArrowRight";
const DEFAULT_PREV_LINE_SHORTCUT: &str = "Control+Alt+ArrowLeft";
const DEFAULT_BOSS_KEY_SHORTCUT: &str = "Control+Alt+ArrowDown";

pub fn reset_app_store(app_handle: &AppHandle) -> Result<(), String> {
    let store = app_handle
        .store("app_data.json")
        .map_err(|e| e.to_string())?;

    store.set(AppStoreKey::DockVisibility.as_str(), false);
    store.set(AppStoreKey::AlwaysOnTop.as_str(), false);
    store.set(AppStoreKey::Transparent.as_str(), true);
    store.set(AppStoreKey::FontSize.as_str(), 16);
    store.set(AppStoreKey::LineHeight.as_str(), 1.5);
    store.set(AppStoreKey::LetterSpacing.as_str(), 0);
    store.set(AppStoreKey::FontWeight.as_str(), 400);
    store.set(AppStoreKey::FontColor.as_str(), "#000000");

    AppShortcut::unregister_all_shortcuts(app_handle)?;

    store.set(
        AppStoreKey::NextLineShortcut.as_str(),
        DEFAULT_NEXT_LINE_SHORTCUT,
    );
    store.set(
        AppStoreKey::PrevLineShortcut.as_str(),
        DEFAULT_PREV_LINE_SHORTCUT,
    );
    store.set(
        AppStoreKey::BossKeyShortcut.as_str(),
        DEFAULT_BOSS_KEY_SHORTCUT,
    );

    set_next_line_shortcut(app_handle.clone(), DEFAULT_NEXT_LINE_SHORTCUT.to_string()).unwrap();
    set_prev_line_shortcut(app_handle.clone(), DEFAULT_PREV_LINE_SHORTCUT.to_string()).unwrap();
    set_boss_key_shortcut(app_handle.clone(), DEFAULT_BOSS_KEY_SHORTCUT.to_string()).unwrap();

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

    Ok(())
}
