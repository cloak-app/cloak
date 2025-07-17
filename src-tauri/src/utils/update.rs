use std::sync::Mutex;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tauri::{async_runtime::JoinHandle, AppHandle, Emitter, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::{Update, UpdaterExt};
use tokio::time::{sleep_until, Duration, Instant};

use crate::{constants::event::*, state::model::AppState, utils::window::open_update_window};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CheckUpdateStatus {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    pub timestamp: i64,
    pub status: CheckUpdateStatus,
}

pub struct UpdateChecker {
    pub last_check_result: Option<UpdateCheckResult>,
    handle: Option<JoinHandle<()>>,
}

impl UpdateChecker {
    pub fn new() -> Self {
        Self {
            last_check_result: None,
            handle: None,
        }
    }

    pub fn start(&mut self, app_handle: &AppHandle, interval: u64) -> Result<(), String> {
        self.stop();

        if interval == 0 {
            return Ok(());
        };

        let app_handle = app_handle.clone();

        let handle = tauri::async_runtime::spawn(async move {
            loop {
                let next_check_time = Instant::now() + Duration::from_secs(interval);
                sleep_until(next_check_time).await;
                Self::check_update(&app_handle).await;
            }
        });

        self.handle = Some(handle);

        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort()
        }
    }

    pub async fn check_update(app_handle: &AppHandle) {
        let timestamp = Utc::now().timestamp();
        log::info!(target: "UpdateChecker", "开始检查更新");

        let checking_result = UpdateCheckResult {
            timestamp,
            status: CheckUpdateStatus::Checking,
        };

        Self::update_check_result(app_handle, checking_result);

        let update = app_handle.updater().unwrap().check().await;

        if let Ok(update) = update {
            let success_result = UpdateCheckResult {
                timestamp,
                status: CheckUpdateStatus::Success,
            };

            Self::update_check_result(app_handle, success_result);

            if let Some(update) = update {
                log::info!(target: "UpdateChecker", "发现新版本{}", update.version);

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
                    log::info!(target: "UpdateChecker", "开始下载并安装更新");
                    Self::download_and_install(app_handle, update).await;
                    log::info!(target: "UpdateChecker", "完成下载并安装更新");
                } else {
                    log::info!(target: "UpdateChecker", "用户选择不更新");
                }
            }
        } else {
            log::error!(target: "UpdateChecker", "检查更新失败");

            let failed_result = UpdateCheckResult {
                timestamp,
                status: CheckUpdateStatus::Failed,
            };

            Self::update_check_result(app_handle, failed_result);
        }

        log::info!(target: "UpdateChecker", "检查更新完成");
    }

    async fn download_and_install(app_handle: &AppHandle, update: Update) {
        open_update_window(app_handle).unwrap();

        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;

                    if let Some(content_length) = content_length {
                        let progress = (downloaded as f64 / content_length as f64) * 100.0;
                        app_handle.emit(UPDATE_PROGRESS_CHANGE, progress).unwrap();
                    }
                },
                || {
                    app_handle.emit(UPDATE_FINISHED, ()).unwrap();
                },
            )
            .await
            .unwrap();

        app_handle.restart();
    }

    fn update_check_result(app_handle: &AppHandle, result: UpdateCheckResult) {
        let state = app_handle.state::<Mutex<AppState>>();
        let mut state = state.lock().unwrap();
        let update_checker = &mut state.update_checker;
        update_checker.last_check_result = Some(result);

        // 通知前端状态变化
        app_handle.emit(UPDATE_CHECK_STATUS_CHANGE, result).unwrap();
    }
}
