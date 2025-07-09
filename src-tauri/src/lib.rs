mod commands;
mod constants;
mod db;
mod state;
mod store;
mod utils;

use crate::commands::{common, config, novel, os, reader, window};
use crate::db::setup_db;
use crate::state::{model::AppState, toggle_reading_mode};
use crate::store::{get_from_app_store, init_app_store, model::AppStoreKey};
use crate::utils::{icon::*, reader::NovelReader, shortcut, sql, update::UpdateChecker, window::*};
use log::LevelFilter;
use std::sync::Mutex;
use tauri::{
    is_dev,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager, RunEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(
            tauri_plugin_log::Builder::new()
                .with_colors(ColoredLevelConfig::default())
                .level(if is_dev() {
                    LevelFilter::Info
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
            }
        }))
        .invoke_handler(tauri::generate_handler![
            // 通用
            common::get_reading_mode,
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
            config::reset_config,
            config::get_last_check_result,
            config::check_update,
            config::set_check_update_interval,
            config::set_auto_start,
            config::set_language,
            config::set_theme,
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
            window::reopen_reader_window,
            window::open_update_window,
        ])
        .setup(|app| {
            /* -------------------------------- 初始化全局上下文 -------------------------------- */
            init_app_store(app.handle())?;

            tauri::async_runtime::block_on(async {
                let db = setup_db(app).await;

                let mut novel_reader: Option<NovelReader> = None;

                let line_size =
                    get_from_app_store::<usize>(app.handle(), AppStoreKey::LineSize).unwrap();

                if let Ok(novel) = sql::get_open_novel(&db).await {
                    novel_reader = NovelReader::new(
                        novel.id,
                        novel.path,
                        novel.read_position as usize,
                        line_size,
                    )
                    .ok();
                }

                let interval =
                    get_from_app_store::<u64>(app.handle(), AppStoreKey::CheckUpdateInterval)
                        .unwrap();

                let mut update_checker = UpdateChecker::new();

                update_checker.start(app.handle(), interval).unwrap();

                app.manage(db);
                app.manage(Mutex::new(AppState {
                    novel_reader,
                    reading_mode: false,
                    update_checker,
                }));
            });

            /* --------------------------------- 开启时检查更新 -------------------------------- */

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                UpdateChecker::check_update(&app_handle).await;
            });

            /* --------------------------------- 注册全局快捷键 -------------------------------- */
            shortcut::activate_shortcuts(app.handle(), shortcut::AppShortcut::common_shortcuts())?;

            /* ---------------------------------- 系统设置 ---------------------------------- */
            #[cfg(target_os = "macos")]
            {
                let dock_visibility =
                    get_from_app_store::<bool>(app.handle(), AppStoreKey::DockVisibility).unwrap();
                app.set_dock_visibility(dock_visibility);
            }

            /* --------------------------------- 注册托盘菜单 --------------------------------- */
            let toggle_reading_mode_i = MenuItemBuilder::new("打开阅读模式")
                .id("toggle_reading_mode")
                .build(app)?;
            let open_reader_i = MenuItemBuilder::new("打开阅读器")
                .id("open_reader")
                .build(app)?;
            let settings_i = MenuItemBuilder::new("设置")
                .id("open_settings")
                .build(app)?;
            let quit_i = MenuItemBuilder::new("退出")
                .id("quit")
                .accelerator("CmdOrCtrl+Q")
                .build(app)?;

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
                    "toggle_reading_mode" => {
                        toggle_reading_mode(app_handle).expect("切换阅读模式失败");
                    }
                    "open_reader" => {
                        open_reader_window(app_handle).expect("打开阅读器窗口失败");
                    }
                    "open_settings" => {
                        open_settings_window(app_handle).expect("打开设置窗口失败");
                    }
                    "quit" => {
                        app_handle.exit(0);
                    }
                    _ => unreachable!(),
                })
                .build(app)?;

            app.manage(menu);

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
