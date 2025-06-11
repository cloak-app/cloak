use crate::commands::novel;
use crate::db::Db;
use crate::state::{AppState, AppStoreKey};
use crate::utils::{get_from_app_store, hide_all_windows};
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

        // 先注销之前的快捷键
        app_handle
            .global_shortcut()
            .unregister(shortcut)
            .map_err(|err| format!("Failed to unregister previous shortcut '{}'", err))?;

        // 新增快捷键
        app_handle
            .global_shortcut()
            .on_shortcut(shortcut, move |app_handle, scut, event| {
                let db = app_handle.state::<Db>();
                let state = app_handle.state::<Mutex<AppState>>();

                if scut == &shortcut && ShortcutState::Pressed == event.state() {
                    match app_shortcut {
                        AppShortcut::NextLine(_) => {
                            tauri::async_runtime::block_on(novel::next_line(db, state)).unwrap();
                        }
                        AppShortcut::PrevLine(_) => {
                            tauri::async_runtime::block_on(novel::prev_line(db, state)).unwrap();
                        }
                        AppShortcut::BossKey(_) => {
                            hide_all_windows(app_handle).unwrap();
                        }
                    }
                }
            })
            .map_err(|err| format!("Failed to register new shortcut '{}'", err))?;

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
