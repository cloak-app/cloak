mod commands;
mod db;
mod reader;
mod state;

use crate::commands::{config, novel};
use crate::db::setup_db;
use crate::reader::NovelReader;
use crate::state::AppState;
use std::sync::Mutex;
use tauri::{menu::*, tray::TrayIconBuilder, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_store::StoreExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            novel::add_novel,
            novel::get_novel_list,
            novel::open_novel,
            novel::get_current_novel,
            novel::get_chapter_list,
            novel::get_line,
            novel::next_line,
            novel::prev_line,
            config::set_dock_visibility,
            config::set_always_on_top,
        ])
        .setup(|app| {
            /* -------------------------------- 初始化全局上下文 -------------------------------- */
            tauri::async_runtime::block_on(async {
                let db = setup_db(app).await;

                let store = app.store("app_data.json").unwrap();

                let last_read_novel_id = store.get("last_read_novel_id").and_then(|v| v.as_i64());

                let mut novel_reader: Option<NovelReader> = None;

                // 上次阅读的小说，如果存在，则打开它
                if let Some(last_read_novel_id) = last_read_novel_id {
                    if let Ok(novel) = novel::get_novel_by_id(&db, last_read_novel_id).await {
                        let reader = NovelReader::new(novel);

                        if let Ok(reader) = reader {
                            novel_reader = Some(reader);
                        }
                    }
                }

                app.manage(db);
                app.manage(store);
                app.manage(Mutex::new(AppState { novel_reader }));
            });

            /* ---------------------------------- 系统设置 ---------------------------------- */
            #[cfg(target_os = "macos")]
            {
                let store = app.get_store("app_data.json").unwrap();
                let dock_visibility = store
                    .get("dock_visibility")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                app.set_dock_visibility(dock_visibility);
            }

            /* --------------------------------- 注册托盘菜单 --------------------------------- */

            let open_reader_i =
                MenuItem::with_id(app, "open_reader", "打开阅读器", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

            let menu = MenuBuilder::new(app)
                .item(&open_reader_i)
                .item(&settings_i)
                .separator()
                .item(&quit_i)
                .build()?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "settings" => {
                        let window = app.get_webview_window("settings");

                        if let Some(window) = window {
                            window.set_focus().unwrap();
                        } else {
                            WebviewWindowBuilder::new(app, "settings", WebviewUrl::default())
                                .inner_size(800.0, 600.0)
                                .build()
                                .unwrap();
                        }
                    }
                    "open_reader" => {
                        let window = app.get_webview_window("reader");

                        if let Some(window) = window {
                            window.set_focus().unwrap();
                        } else {
                            let store = app.get_store("app_data.json").unwrap();
                            let always_on_top = store
                                .get("always_on_top")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);

                            let transparent = store
                                .get("transparent")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(true);

                            WebviewWindowBuilder::new(app, "reader", WebviewUrl::default())
                                .shadow(false)
                                .transparent(transparent)
                                .always_on_top(always_on_top)
                                .inner_size(200.0, 100.0)
                                .build()
                                .unwrap();
                        }
                    }

                    _ => {
                        panic!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, event| {
            if let RunEvent::ExitRequested { api, code, .. } = event {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
        });
}
