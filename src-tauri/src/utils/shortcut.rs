use crate::commands::reader;
use crate::db::Db;
use crate::state::model::AppState;
use crate::store::get_from_app_store;
use crate::store::model::AppStoreKey;
use crate::utils::window::{hide_all_windows, show_all_windows};
use std::sync::Mutex;
use tauri::{App, AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub enum AppShortcut {
    NextLine(Shortcut),
    PrevLine(Shortcut),
    BossKey(Shortcut),
}

impl AppShortcut {
    pub fn init(app: &App) -> Result<(), String> {
        let app_handle = app.handle();

        const SHORTCUTS: [AppStoreKey; 3] = [
            AppStoreKey::NextLineShortcut,
            AppStoreKey::PrevLineShortcut,
            AppStoreKey::BossKeyShortcut,
        ];

        for key in SHORTCUTS {
            get_from_app_store::<String>(app_handle, key)?
                .map(|v| v.parse::<Shortcut>())
                .transpose()
                .map_err(|e| e.to_string())?
                .map(move |shortcut| {
                    let app_shortcut = match key {
                        AppStoreKey::NextLineShortcut => AppShortcut::NextLine(shortcut),
                        AppStoreKey::PrevLineShortcut => AppShortcut::PrevLine(shortcut),
                        AppStoreKey::BossKeyShortcut => AppShortcut::BossKey(shortcut),
                        _ => unreachable!(),
                    };

                    AppShortcut::register_shortcut(app_handle, app_shortcut).unwrap();
                });
        }

        Ok(())
    }

    pub fn register_shortcut(
        app_handle: &AppHandle,
        app_shortcut: AppShortcut,
    ) -> Result<(), String> {
        let shortcut = app_shortcut.get_shortcut();

        app_handle
            .global_shortcut()
            .on_shortcut(shortcut, move |app_handle, scut, event| {
                let db = app_handle.state::<Db>();
                let state = app_handle.state::<Mutex<AppState>>();

                if scut == &shortcut && ShortcutState::Pressed == event.state() {
                    match app_shortcut {
                        AppShortcut::NextLine(_) => {
                            tauri::async_runtime::block_on(reader::next_line(
                                app_handle.clone(),
                                db,
                                state,
                            ))
                            .unwrap();
                        }
                        AppShortcut::PrevLine(_) => {
                            tauri::async_runtime::block_on(reader::prev_line(
                                app_handle.clone(),
                                db,
                                state,
                            ))
                            .unwrap();
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
                    }
                }
            })
            .map_err(|err| format!("Failed to register new shortcut '{}'", err))?;

        Ok(())
    }

    pub fn unregister_shortcut(app_handle: &AppHandle, shortcut: Shortcut) -> Result<(), String> {
        app_handle
            .global_shortcut()
            .unregister(shortcut)
            .map_err(|err| format!("Failed to unregister previous shortcut '{}'", err))?;

        Ok(())
    }

    pub fn unregister_all_shortcuts(app_handle: &AppHandle) -> Result<(), String> {
        app_handle
            .global_shortcut()
            .unregister_all()
            .map_err(|err| format!("Failed to unregister all shortcuts '{}'", err))?;

        Ok(())
    }
    fn get_shortcut(&self) -> Shortcut {
        match self {
            Self::NextLine(shortcut) => *shortcut,
            Self::PrevLine(shortcut) => *shortcut,
            Self::BossKey(shortcut) => *shortcut,
        }
    }
}
