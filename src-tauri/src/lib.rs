mod commands;
mod db;
mod state;
mod store;
mod utils;

use crate::commands::{config, novel, os, reader, window};
use crate::db::setup_db;
use crate::state::model::AppState;
use crate::state::toggle_reading_mode;
use crate::store::model::AppStoreKey;
use crate::store::{get_from_app_store, init_app_store};
use crate::utils::icon::*;
use crate::utils::reader::NovelReader;
use crate::utils::shortcut;
use crate::utils::sql;
use crate::utils::update::update;
use crate::utils::window::{open_reader_window, open_settings_window, show_all_windows};
use log::LevelFilter;
use std::sync::Mutex;
use tauri::{is_dev, menu::*, tray::TrayIconBuilder, Manager, RunEvent};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .with_colors(ColoredLevelConfig::default())
                .level(if is_dev() {
                    LevelFilter::Debug
                } else {
                    LevelFilter::Warn
                })
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app_handle, _, _| {
            let windows = app_handle.webview_windows();
            let has_show_window = windows
                .iter()
                .any(|(_, window)| window.is_visible().unwrap());

            if has_show_window {
                show_all_windows(app_handle).unwrap();
            } else {
                app_handle
                    .dialog()
                    .message("Cloak 已经在运行中，请从托盘打开窗口")
                    .kind(MessageDialogKind::Info)
                    .title("Cloak 正在运行")
                    .buttons(MessageDialogButtons::OkCustom("我知道了".to_string()))
                    .show(|_| ());
            }
        }))
        .invoke_handler(tauri::generate_handler![
            // 小说相关
            novel::add_novel,
            novel::get_novel_list,
            novel::open_novel,
            novel::close_novel,
            novel::delete_novel,
            novel::get_novel_detail,
            // 阅读相关
            reader::get_novel_reader,
            reader::close_novel_reader,
            reader::get_line,
            reader::set_read_position,
            // 配置相关
            config::get_config,
            config::set_dock_visibility,
            config::set_always_on_top,
            config::set_transparent,
            config::set_line_size,
            config::set_font_size,
            config::set_font_family,
            config::set_line_height,
            config::set_font_weight,
            config::set_font_color,
            config::set_letter_spacing,
            config::reset_config,
            // 快捷键相关
            config::set_next_line_shortcut,
            config::set_prev_line_shortcut,
            config::set_next_chapter_shortcut,
            config::set_prev_chapter_shortcut,
            config::set_boss_key_shortcut,
            config::set_toggle_reading_mode_shortcut,
            config::activate_all_shortcuts,
            config::unregister_all_shortcuts,
            // 系统相关
            os::reveal_item_in_dir,
            os::get_all_font_families,
            // 窗口相关
            window::open_reader_window,
        ])
        .setup(|app| {
            /* -------------------------------- 初始化全局上下文 -------------------------------- */
            init_app_store(app.handle())?;

            tauri::async_runtime::block_on(async {
                let db = setup_db(app).await;

                let mut novel_reader: Option<NovelReader> = None;

                let line_size =
                    get_from_app_store::<usize>(app.handle(), AppStoreKey::LineSize).unwrap_or(50);

                if let Ok(novel) = sql::get_open_novel(&db).await {
                    novel_reader = NovelReader::new(
                        novel.id,
                        novel.path,
                        novel.read_position as usize,
                        line_size,
                    )
                    .ok();
                }

                app.manage(db);
                app.manage(Mutex::new(AppState {
                    novel_reader,
                    reading_mode: false,
                }));
            });

            /* --------------------------------- 注册全局快捷键 -------------------------------- */
            shortcut::activate_shortcuts(app.handle(), shortcut::AppShortcut::common_shortcuts())?;

            /* ---------------------------------- 系统设置 ---------------------------------- */
            #[cfg(target_os = "macos")]
            {
                let dock_visibility =
                    get_from_app_store::<bool>(app.handle(), AppStoreKey::DockVisibility)
                        .unwrap_or(false);
                app.set_dock_visibility(dock_visibility);
            }

            /* --------------------------------- 注册托盘菜单 --------------------------------- */
            let toggle_reading_mode_i = MenuItem::with_id(
                app,
                "toggle_reading_mode",
                "打开阅读模式",
                true,
                None::<&str>,
            )?;
            let open_reader_i =
                MenuItem::with_id(app, "open_reader", "打开阅读器", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

            let menu = MenuBuilder::new(app)
                .item(&toggle_reading_mode_i)
                .separator()
                .item(&open_reader_i)
                .item(&settings_i)
                .separator()
                .item(&quit_i)
                .build()?;

            let default_tray_icon = get_default_tray_icon(app.handle()).unwrap();

            TrayIconBuilder::with_id("tray")
                .icon(default_tray_icon)
                .menu(&menu)
                .on_menu_event(move |app_handle, event| match event.id.as_ref() {
                    "quit" => {
                        app_handle.exit(0);
                    }
                    "settings" => {
                        open_settings_window(app_handle).expect("打开设置窗口失败");
                    }
                    "open_reader" => {
                        open_reader_window(app_handle).expect("打开阅读器窗口失败");
                    }
                    "toggle_reading_mode" => {
                        toggle_reading_mode(app_handle).expect("切换阅读模式失败");
                    }
                    _ => unreachable!(),
                })
                .build(app)?;

            app.manage(menu);

            /* ---------------------------------- 检查更新 ---------------------------------- */
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(app_handle).await.unwrap();
            });

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("运行 Tauri 应用失败")
        .run(|_, event| {
            if let RunEvent::ExitRequested { api, code, .. } = event {
                if code.is_none() {
                    api.prevent_exit();
                }
            }
        });
}
