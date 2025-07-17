pub mod model;

use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::{constants::store::*, store::model::AppStoreKey};

pub fn get_entries_from_app_store(app_handle: &AppHandle) -> Result<Vec<(String, Value)>, String> {
    let store = app_handle
        .store(APP_STORE_PATH)
        .map_err(|e| e.to_string())?;

    let entries = store.entries();

    Ok(entries)
}

pub fn reset_app_store(app_handle: &AppHandle) -> Result<(), String> {
    let store = app_handle
        .store(APP_STORE_PATH)
        .map_err(|e| e.to_string())?;

    AppStoreKey::keys().iter().for_each(|key| {
        store.set(key.as_str(), key.default_value());
    });

    store.save().unwrap();

    Ok(())
}

pub fn init_app_store(app_handle: &AppHandle) -> Result<(), String> {
    let store = app_handle
        .store(APP_STORE_PATH)
        .map_err(|e| e.to_string())?;

    let store_keys = store.keys();

    AppStoreKey::keys().iter().for_each(|key| {
        let key_string = key.as_str().to_string();
        if !store_keys.contains(&key_string) {
            store.set(&key_string, key.default_value());
        }
    });

    store.save().unwrap();

    Ok(())
}

pub fn get_from_app_store<T: serde::de::DeserializeOwned>(
    app_handle: &AppHandle,
    key: AppStoreKey,
) -> Option<T> {
    if let Ok(store) = app_handle.store(APP_STORE_PATH) {
        let json_value = store.get(key.as_str())?;
        if let Ok(value) = serde_json::from_value(json_value) {
            return Some(value);
        }
    }

    None
}

pub fn set_to_app_store<T: serde::Serialize>(
    app_handle: &AppHandle,
    key: AppStoreKey,
    value: T,
) -> Result<(), String> {
    let store = app_handle
        .store(APP_STORE_PATH)
        .map_err(|e| e.to_string())?;

    let json_value = serde_json::to_value(value).map_err(|e| e.to_string())?;
    store.set(key.as_str(), json_value);
    store.save().map_err(|e| e.to_string())?;

    Ok(())
}
