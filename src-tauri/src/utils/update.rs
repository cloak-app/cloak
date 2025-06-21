use crate::utils::window::open_update_window;
use tauri::{AppHandle, Emitter};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

pub async fn update(app_handle: AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app_handle.updater()?.check().await? {
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
                .await?;

            let answer = app_handle
                .dialog()
                .message("更新完成，是否重启？")
                .title("检查更新")
                .buttons(MessageDialogButtons::OkCancelCustom(
                    "立即重启".to_string(),
                    "下次再说".to_string(),
                ))
                .blocking_show();

            if answer {
                app_handle.restart();
            }
        }
    }

    Ok(())
}
