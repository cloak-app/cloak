use crate::utils::state::AppStoreKey;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

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
