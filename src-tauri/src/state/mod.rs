pub mod model;

use crate::state::model::AppState;
use crate::utils::shortcut::{self, AppShortcut};
use std::sync::Mutex;
use tauri::{image::Image, menu::Menu, path::BaseDirectory, AppHandle, Manager, Wry};

pub static DEFAULT_TRAY_ICON_PATH: &str = "icons/tray-icon.ico";
pub static ACTIVE_TRAY_ICON_PATH: &str = "icons/tray-icon-active.ico";

pub struct TrayIcon {}

impl TrayIcon {
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
}

pub fn toggle_reading_mode(app_handle: &AppHandle) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut state = state.lock().map_err(|e| e.to_string())?;

    state.reading_mode = !state.reading_mode;

    let tray_icon = app_handle.tray_by_id("tray").ok_or("获取托盘图标失败")?;
    let menu = app_handle.state::<Menu<Wry>>();

    let toggle_reading_mode_i = menu
        .get("toggle_reading_mode")
        .ok_or("获取阅读模式菜单项失败")?;

    if !state.reading_mode {
        tray_icon
            .set_icon(TrayIcon::get_default_tray_icon(app_handle).ok())
            .map_err(|e| format!("设置默认托盘图标失败: {e}"))?;
        toggle_reading_mode_i
            .as_menuitem()
            .map(|menu_item| menu_item.set_text("打开阅读模式"));

        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) =
                shortcut::deactivate_shortcuts(&app_handle, AppShortcut::reading_mode_shortcuts())
            {
                log::error!(target: "toggle_reading_mode", "移除快捷键失败: {e}");
            }
        });
    } else {
        tray_icon
            .set_icon(TrayIcon::get_active_tray_icon_path(app_handle).ok())
            .map_err(|e| format!("设置阅读态托盘图标失败: {e}"))?;
        toggle_reading_mode_i
            .as_menuitem()
            .map(|menu_item| menu_item.set_text("关闭阅读模式"));

        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            if let Err(e) =
                shortcut::activate_shortcuts(&app_handle, AppShortcut::reading_mode_shortcuts())
            {
                log::error!(target: "toggle_reading_mode", "激活快捷键失败: {e}");
            }
        });
    }

    Ok(())
}
