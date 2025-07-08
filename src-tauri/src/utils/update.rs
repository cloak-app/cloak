use crate::state::model::AppState;
use crate::utils::window::open_update_window;
use chrono::Utc;
use std::sync::Mutex;
use tauri::{async_runtime::JoinHandle, AppHandle, Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::{Update, UpdaterExt};
use tokio::time::{sleep_until, Duration, Instant};

pub struct UpdateChecker {
    pub last_check_time: Option<i64>,
    handle: Option<JoinHandle<()>>,
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            last_check_time: None,
            handle: None,
        }
    }

    pub fn start(&mut self, app_handle: &AppHandle, interval: u64) -> Result<(), String> {
        self.stop();

        if interval == 0 {
            return Ok(());
        };

        self.check(app_handle).unwrap();

        let app_handle = app_handle.clone();

        let handle = tauri::async_runtime::spawn(async move {
            loop {
                log::info!(target: "UpdateChecker", "检查更新");
                let next_check_time = Instant::now() + Duration::from_secs(interval);
                sleep_until(next_check_time).await;

                let state = app_handle.state::<Mutex<AppState>>();
                let mut state = state.lock().unwrap();
                let update_checker = &mut state.update_checker;

                update_checker.check(&app_handle).unwrap();
            }
        });

        self.handle = Some(handle);

        Ok(())
    }

    pub fn stop(&mut self) {
        self.handle.take().map(|handle| handle.abort());
    }

    fn check(&mut self, app_handle: &AppHandle) -> Result<(), String> {
        let timestamp = Utc::now().timestamp();
        self.last_check_time = Some(timestamp);

        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            if let Ok(updater) = app_handle.updater() {
                if let Ok(Some(update)) = updater.check().await {
                    let answer = app_handle
                        .dialog()
                        .message(format!("发现新版本{}，是否更新？", update.version))
                        .title("检查更新")
                        .buttons(MessageDialogButtons::OkCancelCustom(
                            "立即更新".to_string(),
                            "下次再说".to_string(),
                        ))
                        .blocking_show();

                    if answer {
                        download_and_install(&app_handle, update).await;
                    }
                }
            }
        });

        Ok(())
    }
}

async fn download_and_install(app_handle: &AppHandle, update: Update) {
    open_update_window(&app_handle).unwrap();

    let mut downloaded = 0;

    update
        .download_and_install(
            |chunk_length, content_length| {
                downloaded += chunk_length;

                if let Some(content_length) = content_length {
                    let progress = (downloaded as f64 / content_length as f64) * 100.0;
                    app_handle.emit("update-progress-change", progress).unwrap();
                }
            },
            || {
                app_handle.emit("update-finished", ()).unwrap();
            },
        )
        .await
        .unwrap();

    app_handle.restart();
}
