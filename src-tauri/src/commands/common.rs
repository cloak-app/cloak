use std::sync::Mutex;

use tauri::Manager;

use crate::state::model::AppState;

#[tauri::command]
pub fn get_reading_mode(app_handle: tauri::AppHandle) -> Result<bool, String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let state = state.lock().unwrap();
    Ok(state.reading_mode)
}
