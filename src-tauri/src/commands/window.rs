use tauri::AppHandle;

use crate::utils;

#[tauri::command]
pub fn open_reader_window(app_handle: AppHandle) -> Result<(), String> {
    utils::window::open_reader_window(&app_handle)?;
    Ok(())
}
