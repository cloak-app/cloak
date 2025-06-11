mod commands;
mod db;
mod utils;

use crate::commands::{config, novel, reader};
use crate::db::setup_db;
use crate::utils::novel::get_novel_by_id;
use crate::utils::reader::NovelReader;
use crate::utils::shortcut::AppShortcut;
use crate::utils::state::{AppState, AppStoreKey};
use crate::utils::store::get_from_app_store;
use crate::utils::window::{open_reader_window, open_settings_window};
use std::sync::Mutex;
use tauri::{menu::*, tray::TrayIconBuilder, Manager, RunEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // 小说相关
            novel::add_novel,
            novel::get_novel_list,
            novel::open_novel,
            // 阅读相关
            reader::get_novel_reader,
            reader::get_line,
            reader::set_line_num,
            reader::next_line,
            reader::prev_line,
            // 配置相关
            config::get_config,
            config::set_dock_visibility,
            config::set_always_on_top,
            config::set_transparent,
            config::set_font_size,
            config::set_font_family,
            config::set_line_height,
            config::set_font_weight,
            config::set_font_color,
            config::set_background_color,
            config::set_next_line_shortcut,
            config::set_prev_line_shortcut,
            config::set_boss_key_shortcut,
            config::unset_shortcut,
        ])
        .setup(|app| {
            /* -------------------------------- 初始化全局上下文 -------------------------------- */
            tauri::async_runtime::block_on(async {
                let db = setup_db(app).await;

                let mut novel_reader: Option<NovelReader> = None;

                let last_read_novel_id =
                    get_from_app_store::<i64>(app.handle(), AppStoreKey::LastReadNovelId);
                if let Ok(Some(id)) = last_read_novel_id {
                    let novel = get_novel_by_id(&db, id).await.ok();
                    if let Some(novel) = novel {
                        novel_reader = NovelReader::new(novel).ok();
                    }
                }

                app.manage(db);
                app.manage(Mutex::new(AppState { novel_reader }));
            });

            /* --------------------------------- 注册全局快捷键 -------------------------------- */
            AppShortcut::init(app).unwrap_or_else(|e| {
                println!("Failed to register global shortcuts: {}", e);
            });

            /* ---------------------------------- 系统设置 ---------------------------------- */
            #[cfg(target_os = "macos")]
            {
                let dock_visibility =
                    get_from_app_store::<bool>(app.handle(), AppStoreKey::DockVisibility)?
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
                .on_menu_event(|app_handle, event| match event.id.as_ref() {
                    "quit" => {
                        app_handle.exit(0);
                    }
                    "settings" => {
                        open_settings_window(app_handle).unwrap();
                    }
                    "open_reader" => {
                        open_reader_window(app_handle).unwrap();
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
