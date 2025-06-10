use crate::db::model::Novel;
use crate::db::Db;
use crate::state::AppStoreKey;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;

/* ---------------------------------- db 相关 --------------------------------- */
pub async fn get_novel_by_id(db: &Db, id: i64) -> Result<Novel, String> {
    let novel = sqlx::query_as::<_, Novel>("SELECT * FROM novel WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| format!("Error fetching novel: {}", e))?;

    Ok(novel)
}

pub async fn save_novel(db: &Db, novel_id: i64, last_read_position: i64) -> Result<(), String> {
    sqlx::query("UPDATE novel SET last_read_position = ? WHERE id = ?")
        .bind(last_read_position)
        .bind(novel_id)
        .execute(db)
        .await
        .map_err(|e| format!("Error saving novel: {}", e))?;

    Ok(())
}

/* -------------------------------- store 相关 -------------------------------- */
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

/* -------------------------------- window 相关 ------------------------------- */
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
