use tauri::AppHandle;

use crate::utils::window;

#[tauri::command]
pub fn open_reader_window(app_handle: AppHandle) -> Result<(), String> {
    window::open_reader_window(&app_handle)?;
    Ok(())
}

#[tauri::command]
pub fn open_update_window(app_handle: AppHandle) -> Result<(), String> {
    window::open_update_window(&app_handle)?;
    Ok(())
}
