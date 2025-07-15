use tauri::AppHandle;
use tokio::time::{sleep, Duration};

use crate::utils::window;

#[tauri::command]
pub async fn open_reader_window(app_handle: AppHandle) -> Result<(), String> {
    window::open_reader_window(&app_handle)?;
    Ok(())
}

#[tauri::command]
pub async fn reopen_reader_window(app_handle: AppHandle) -> Result<(), String> {
    window::destroy_reader_window(&app_handle)?;

    sleep(Duration::from_secs(2)).await;

    window::open_reader_window(&app_handle)?;
    Ok(())
}

#[tauri::command]
pub async fn open_update_window(app_handle: AppHandle) -> Result<(), String> {
    window::open_update_window(&app_handle)?;
    Ok(())
}
