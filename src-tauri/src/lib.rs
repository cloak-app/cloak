mod commands;
mod db;
mod state;

use crate::commands::novel;
use crate::db::setup_db;
use crate::state::AppState;
use std::sync::Mutex;
use tauri::{menu::*, tray::TrayIconBuilder, Manager, RunEvent, WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            novel::add_novel,
            novel::get_novel_list,
            novel::open_novel,
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_dock_visibility(false);

            /* --------------------------------- 新建数据库连接 -------------------------------- */
            tauri::async_runtime::block_on(async {
                let db = setup_db(app).await;
                app.manage(db);
                app.manage(Mutex::new(AppState { novel_reader: None }));
            });

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
                        if let None = window {
                            WebviewWindowBuilder::new(app, "settings", WebviewUrl::default())
                                .inner_size(400.0, 200.0)
                                .build()
                                .unwrap();
                        } else {
                            window.unwrap().set_focus().unwrap();
                        }
                    }
                    "open_reader" => {
                        let window = app.get_webview_window("reader");
                        if let None = window {
                            WebviewWindowBuilder::new(app, "reader", WebviewUrl::default())
                                .shadow(false)
                                .transparent(true)
                                .inner_size(200.0, 100.0)
                                .build()
                                .unwrap();
                        } else {
                            window.unwrap().set_focus().unwrap();
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
