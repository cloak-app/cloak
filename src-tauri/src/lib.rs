mod commands;
mod db;
mod state;
mod store;
mod utils;

use crate::commands::{config, novel, reader};
use crate::db::setup_db;
use crate::state::model::AppState;
use crate::store::model::AppStoreKey;
use crate::store::{get_from_app_store, init_app_store};
use crate::utils::novel::get_novel_by_id;
use crate::utils::reader::NovelReader;
use crate::utils::shortcut::AppShortcut;
use crate::utils::update::update;
use crate::utils::window::{open_reader_window, open_settings_window, show_all_windows};
use std::sync::Mutex;
use tauri::image::Image;
use tauri::menu::*;
use tauri::tray::{TrayIconBuilder, TrayIconId};
use tauri::{Manager, RunEvent};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

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
            // 阅读相关
            reader::get_novel_reader,
            reader::close_novel_reader,
            reader::get_line,
            reader::set_line_num,
            // 配置相关
            config::get_config,
            config::set_dock_visibility,
            config::set_always_on_top,
            config::set_transparent,
            config::set_line_size,
            config::set_font_size,
            config::get_all_font_families,
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
        ])
        .setup(|app| {
            /* -------------------------------- 初始化全局上下文 -------------------------------- */
            init_app_store(app.handle()).unwrap();

            tauri::async_runtime::block_on(async {
                let db = setup_db(app).await;

                let mut novel_reader: Option<NovelReader> = None;

                let last_read_novel_id =
                    get_from_app_store::<i64>(app.handle(), AppStoreKey::LastReadNovelId);

                let line_size = get_from_app_store::<usize>(app.handle(), AppStoreKey::LineSize);

                if let (Ok(Some(id)), Ok(Some(line_size))) = (last_read_novel_id, line_size) {
                    let novel = get_novel_by_id(&db, id).await.ok();
                    if let Some(novel) = novel {
                        novel_reader = NovelReader::new(novel, line_size).ok();
                    }
                }

                app.manage(db);
                app.manage(Mutex::new(AppState {
                    novel_reader,
                    reading_mode: false,
                }));
            });

            /* --------------------------------- 注册全局快捷键 -------------------------------- */
            AppShortcut::activate_shortcuts(app.handle(), vec![AppStoreKey::BossKeyShortcut])
                .unwrap();

            /* ---------------------------------- 系统设置 ---------------------------------- */
            #[cfg(target_os = "macos")]
            {
                let dock_visibility =
                    get_from_app_store::<bool>(app.handle(), AppStoreKey::DockVisibility)?
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

            let tray_icon = TrayIconBuilder::new()
                .icon(Image::from_path("../public/tray-icon.ico").unwrap())
                .menu(&menu)
                .on_menu_event(move |app_handle, event| match event.id.as_ref() {
                    "quit" => {
                        app_handle.exit(0);
                    }
                    "settings" => {
                        open_settings_window(app_handle).unwrap();
                    }
                    "open_reader" => {
                        open_reader_window(app_handle).unwrap();
                    }
                    "toggle_reading_mode" => {
                        let state = app_handle.state::<Mutex<AppState>>();
                        let mut state = state.lock().unwrap();
                        state.reading_mode = !state.reading_mode;

                        let tray_icon_id = app_handle.state::<TrayIconId>();
                        let tray_icon = app_handle.tray_by_id(tray_icon_id.inner()).unwrap();

                        if !state.reading_mode {
                            let icon = Image::from_path("../public/tray-icon.ico").ok();
                            tray_icon.set_icon(icon).unwrap();
                            toggle_reading_mode_i.set_text("打开阅读模式").unwrap();
                            AppShortcut::deactivate_shortcuts(
                                app_handle,
                                vec![
                                    AppStoreKey::NextLineShortcut,
                                    AppStoreKey::PrevLineShortcut,
                                    AppStoreKey::NextChapterShortcut,
                                    AppStoreKey::PrevChapterShortcut,
                                ],
                            )
                            .unwrap();
                        } else {
                            let icon = Image::from_path("../public/tray-icon-active.ico").ok();
                            tray_icon.set_icon(icon).unwrap();
                            toggle_reading_mode_i.set_text("关闭阅读模式").unwrap();
                            AppShortcut::activate_shortcuts(
                                app_handle,
                                vec![
                                    AppStoreKey::NextLineShortcut,
                                    AppStoreKey::PrevLineShortcut,
                                    AppStoreKey::NextChapterShortcut,
                                    AppStoreKey::PrevChapterShortcut,
                                ],
                            )
                            .unwrap();
                        }
                    }
                    _ => {
                        panic!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            app.manage(tray_icon.id().clone());

            /* ---------------------------------- 检查更新 ---------------------------------- */
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(app_handle).await.unwrap();
            });

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
