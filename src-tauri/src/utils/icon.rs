use tauri::{image::Image, path::BaseDirectory, AppHandle, Manager};

static DEFAULT_TRAY_ICON_PATH: &str = "icons/tray-icon.ico";
static ACTIVE_TRAY_ICON_PATH: &str = "icons/tray-icon-active.ico";

pub fn get_default_tray_icon(app_handle: &AppHandle) -> Result<Image, String> {
    let default_tray_icon_path = app_handle
        .path()
        .resolve(DEFAULT_TRAY_ICON_PATH, BaseDirectory::Resource)
        .map_err(|e| format!("默认图标路径解析失败: {e}"))?;

    Image::from_path(default_tray_icon_path).map_err(|e| format!("默认图标读取失败: {e}"))
}

pub fn get_active_tray_icon_path(app_handle: &AppHandle) -> Result<Image, String> {
    let active_tray_icon_path = app_handle
        .path()
        .resolve(ACTIVE_TRAY_ICON_PATH, BaseDirectory::Resource)
        .map_err(|e| format!("激活态图标路径解析失败: {e}"))?;

    Image::from_path(active_tray_icon_path).map_err(|e| format!("激活态图标读取失败: {e}"))
}
