mod commands;
mod db;
mod state;

use crate::commands::novel;
use crate::db::setup_db;
use crate::state::AppState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            novel::add_novel,
            novel::get_novel_list,
            novel::open_novel
        ])
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db = setup_db(app).await;
                app.manage(db);
                app.manage(Mutex::new(AppState { novel_reader: None }));
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
