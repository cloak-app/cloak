use crate::db::model::Novel;
use crate::db::Db;
use crate::state::model::AppState;
use crate::store::model::AppStoreKey;
use crate::store::set_to_app_store;
use crate::utils::novel::get_novel_by_id;
use crate::utils::reader::NovelReader;
use std::fs::{copy, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Mutex;
use tauri::{Emitter, Manager};

#[tauri::command]
pub async fn add_novel(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    path: &str,
) -> Result<(), String> {
    let filepath = Path::new(path);

    // 校验文件是否存在
    if !filepath.exists() {
        return Err("File does not exist".to_string());
    }

    let filename = filepath.file_name().unwrap().to_str().unwrap();

    // 校验文件后缀名为 .txt
    if !filename.ends_with(".txt") {
        return Err("File must have a `.txt` extension".to_string());
    }

    // 获取文件行数
    let file = File::open(path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let total_lines = reader.lines().count() as i64;

    // 将文件复制到应用程序目录
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get data_dir");

    let mut new_path = app_data_dir.clone();
    new_path.push(filename);

    copy(path, &new_path).map_err(|e| format!("Error copying file: {}", e))?;

    let title = filename.split(".").next().ok_or("Invalid filename")?;

    let new_path_str = new_path.to_str().ok_or("Invalid file path")?;

    sqlx::query(
        "INSERT INTO novel (title, path, last_read_position, total_lines) VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(title)
    .bind(new_path_str)
    .bind(0)
    .bind(total_lines)
    .execute(&*db)
    .await
    .map_err(|e| format!("Error executing query: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_novel_list(db: tauri::State<'_, Db>) -> Result<Vec<Novel>, String> {
    let novels = sqlx::query_as::<_, Novel>("SELECT * FROM novel")
        .fetch_all(&*db)
        .await
        .map_err(|e| format!("Error fetching novels: {}", e))?;

    Ok(novels)
}

#[tauri::command]
pub async fn open_novel(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let novel = get_novel_by_id(&*db, id).await?;

    set_to_app_store(&app_handle, AppStoreKey::LastReadNovelId, novel.id)?;

    // 创建 reader 并更新状态
    let reader = NovelReader::new(novel);

    if let Ok(reader) = reader {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        state.novel_reader = Some(reader);

        app_handle.emit("reader-line-num-changed", 0).unwrap();

        Ok(())
    } else {
        Err(format!("Failed to open novel!"))
    }
}
