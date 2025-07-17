use std::sync::Mutex;

use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

use crate::{
    commands::reader,
    db::Db,
    state::{model::AppState, toggle_reading_mode},
    store::{get_from_app_store, model::AppStoreKey},
    utils::window::{hide_all_windows, show_all_windows},
};

#[derive(Debug)]
pub enum AppShortcut {
    NextLine(Shortcut),
    PrevLine(Shortcut),
    BossKey(Shortcut),
    NextChapter(Shortcut),
    PrevChapter(Shortcut),
    ToggleReadingMode(Shortcut),
}

impl AppShortcut {
    pub fn new(key: AppStoreKey, shortcut: Shortcut) -> AppShortcut {
        match key {
            AppStoreKey::NextLineShortcut => Self::NextLine(shortcut),
            AppStoreKey::PrevLineShortcut => Self::PrevLine(shortcut),
            AppStoreKey::BossKeyShortcut => Self::BossKey(shortcut),
            AppStoreKey::NextChapterShortcut => Self::NextChapter(shortcut),
            AppStoreKey::PrevChapterShortcut => Self::PrevChapter(shortcut),
            AppStoreKey::ToggleReadingModeShortcut => Self::ToggleReadingMode(shortcut),
            _ => unreachable!(),
        }
    }

    pub fn get_shortcut(&self) -> Shortcut {
        match self {
            Self::NextLine(shortcut) => *shortcut,
            Self::PrevLine(shortcut) => *shortcut,
            Self::BossKey(shortcut) => *shortcut,
            Self::NextChapter(shortcut) => *shortcut,
            Self::PrevChapter(shortcut) => *shortcut,
            Self::ToggleReadingMode(shortcut) => *shortcut,
        }
    }

    pub fn common_shortcuts() -> Vec<AppStoreKey> {
        vec![
            AppStoreKey::BossKeyShortcut,
            AppStoreKey::ToggleReadingModeShortcut,
        ]
    }

    pub fn reading_mode_shortcuts() -> Vec<AppStoreKey> {
        vec![
            AppStoreKey::NextLineShortcut,
            AppStoreKey::PrevLineShortcut,
            AppStoreKey::NextChapterShortcut,
            AppStoreKey::PrevChapterShortcut,
        ]
    }

    pub fn all_shortcuts() -> Vec<AppStoreKey> {
        vec![
            AppStoreKey::NextLineShortcut,
            AppStoreKey::PrevLineShortcut,
            AppStoreKey::NextChapterShortcut,
            AppStoreKey::PrevChapterShortcut,
            AppStoreKey::BossKeyShortcut,
            AppStoreKey::ToggleReadingModeShortcut,
        ]
    }
}

pub fn activate_shortcuts(app_handle: &AppHandle, keys: Vec<AppStoreKey>) -> Result<(), String> {
    for key in keys {
        if let Some(value) = get_from_app_store::<String>(app_handle, key) {
            let shortcut = value.parse::<Shortcut>().map_err(|e| e.to_string())?;
            let app_shortcut = AppShortcut::new(key, shortcut);
            register_shortcut(app_handle, app_shortcut)?;
        } else {
            return Err("快捷键解释失败".to_string());
        }
    }
    Ok(())
}

pub fn deactivate_shortcuts(app_handle: &AppHandle, keys: Vec<AppStoreKey>) -> Result<(), String> {
    for key in keys {
        if let Some(value) = get_from_app_store::<String>(app_handle, key) {
            let shortcut = value.parse::<Shortcut>().map_err(|e| e.to_string())?;
            let app_shortcut = AppShortcut::new(key, shortcut);
            unregister_shortcut(app_handle, app_shortcut)?;
        } else {
            return Err("快捷键解释失败".to_string());
        }
    }
    Ok(())
}

pub fn unregister_all_shortcuts(app_handle: &AppHandle) -> Result<(), String> {
    app_handle
        .global_shortcut()
        .unregister_all()
        .map_err(|err| format!("取消全部快捷键失败: {err}"))?;

    Ok(())
}

fn register_shortcut(app_handle: &AppHandle, app_shortcut: AppShortcut) -> Result<(), String> {
    let shortcut = app_shortcut.get_shortcut();

    app_handle
        .global_shortcut()
        .on_shortcut(shortcut, move |app_handle, scut, event| {
            let db = app_handle.state::<Db>();
            let state = app_handle.state::<Mutex<AppState>>();

            if scut == &shortcut && ShortcutState::Pressed == event.state() {
                match app_shortcut {
                    AppShortcut::NextLine(_) => {
                        let _ = tauri::async_runtime::block_on(reader::next_line(
                            app_handle.clone(),
                            db,
                            state,
                        ));
                    }
                    AppShortcut::PrevLine(_) => {
                        let _ = tauri::async_runtime::block_on(reader::prev_line(
                            app_handle.clone(),
                            db,
                            state,
                        ));
                    }
                    AppShortcut::NextChapter(_) => {
                        let _ = tauri::async_runtime::block_on(reader::next_chapter(
                            app_handle.clone(),
                            db,
                            state,
                        ));
                    }
                    AppShortcut::PrevChapter(_) => {
                        let _ = tauri::async_runtime::block_on(reader::prev_chapter(
                            app_handle.clone(),
                            db,
                            state,
                        ));
                    }
                    AppShortcut::BossKey(_) => {
                        let windows = app_handle.webview_windows();

                        let has_show_window = windows
                            .iter()
                            .any(|(_, window)| window.is_visible().unwrap());

                        if has_show_window {
                            hide_all_windows(app_handle).unwrap();
                        } else {
                            show_all_windows(app_handle).unwrap();
                        }
                    }
                    AppShortcut::ToggleReadingMode(_) => {
                        let _ = toggle_reading_mode(app_handle);
                    }
                }
            }
        })
        .map_err(|err| format!("注册快捷键失败: {err}"))?;

    Ok(())
}

pub fn unregister_shortcut(
    app_handle: &AppHandle,
    app_shortcut: AppShortcut,
) -> Result<(), String> {
    let shortcut = app_shortcut.get_shortcut();

    app_handle
        .global_shortcut()
        .unregister(shortcut)
        .map_err(|err| format!("取消快捷键失败: {err}"))?;

    Ok(())
}
