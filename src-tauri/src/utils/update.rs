use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_updater::UpdaterExt;

pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let answer = app
            .dialog()
            .message(format!("发现新版本{}，是否更新？", update.version))
            .title("检查更新")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "立即更新".to_string(),
                "下次再说".to_string(),
            ))
            .blocking_show();

        if answer {
            let mut downloaded = 0;

            update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length;
                        println!("downloaded {downloaded} from {content_length:?}");
                    },
                    || {
                        println!("download finished");
                    },
                )
                .await?;

            let answer = app
                .dialog()
                .message("更新完成，是否重启？")
                .title("检查更新")
                .buttons(MessageDialogButtons::OkCancelCustom(
                    "立即重启".to_string(),
                    "下次再说".to_string(),
                ))
                .blocking_show();

            if answer {
                app.restart();
            }
        }
    }

    Ok(())
}
