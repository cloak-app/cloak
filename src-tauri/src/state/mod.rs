pub mod model;

use crate::state::model::AppState;
use crate::utils::icon::*;
use crate::utils::shortcut::{self, AppShortcut};
use std::sync::Mutex;
use tauri::{menu::Menu, AppHandle, Manager, Wry};

pub fn toggle_reading_mode(app_handle: &AppHandle) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.reading_mode = !state.reading_mode;

    let tray_icon = app_handle.tray_by_id("tray").ok_or("获取托盘图标失败")?;
    let menu = app_handle.state::<Menu<Wry>>();

    let toggle_reading_mode_i = menu
        .get("toggle_reading_mode")
        .ok_or("获取阅读模式菜单项失败")?;

    if !state.reading_mode {
        tray_icon
            .set_icon(get_default_tray_icon(app_handle).ok())
            .map_err(|e| format!("设置默认托盘图标失败: {e}"))?;
        toggle_reading_mode_i
            .as_menuitem()
            .map(|menu_item| menu_item.set_text("打开阅读模式"));

        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            match shortcut::deactivate_shortcuts(&app_handle, AppShortcut::reading_mode_shortcuts())
            {
                Ok(_) => {
                    log::info!(target: "toggle_reading_mode", "移除快捷键成功");
                }
                Err(e) => {
                    log::error!(target: "toggle_reading_mode", "移除快捷键失败: {e}");
                }
            }
        });
    } else {
        tray_icon
            .set_icon(get_active_tray_icon_path(app_handle).ok())
            .map_err(|e| format!("设置阅读态托盘图标失败: {e}"))?;
        toggle_reading_mode_i
            .as_menuitem()
            .map(|menu_item| menu_item.set_text("关闭阅读模式"));

        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            match shortcut::activate_shortcuts(&app_handle, AppShortcut::reading_mode_shortcuts()) {
                Ok(_) => {
                    log::info!(target: "toggle_reading_mode", "激活快捷键成功");
                }
                Err(e) => {
                    log::error!(target: "toggle_reading_mode", "激活快捷键失败: {e}");
                }
            }
        });
    }

    Ok(())
}
