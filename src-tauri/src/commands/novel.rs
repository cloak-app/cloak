use crate::db::model::Novel;
use crate::db::Db;
use crate::state::model::AppState;
use crate::store::model::AppStoreKey;
use crate::store::{delete_from_app_store, get_from_app_store, set_to_app_store};
use crate::utils::novel::{delete_novel_from_db, get_novel_by_id};
use crate::utils::reader::NovelReader;
use std::fs::{copy, remove_file};
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

    // 将文件复制到应用程序目录
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get data_dir");

    let mut new_path = app_data_dir.clone();
    new_path.push(filename);

    copy(path, &new_path).map_err(|e| format!("Error copying file: {}", e))?;

    let title = filename.split(".").next().expect("Invalid filename");
    let new_path_str = new_path.to_str().expect("Invalid file path");
    let file_size = filepath
        .metadata()
        .map(|metadata| metadata.len())
        .map_err(|e| format!("Error getting file metadata: {}", e))?;

    sqlx::query(
        "INSERT INTO novel (title, path, read_position, read_progress, file_size) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(title)
    .bind(new_path_str)
    .bind(0)
    .bind(0)
    .bind(file_size as i64)
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
pub async fn get_novel_detail(db: tauri::State<'_, Db>, id: i64) -> Result<Novel, String> {
    let novel = get_novel_by_id(&*db, id).await?;

    Ok(novel)
}

#[tauri::command]
pub async fn open_novel(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let novel = get_novel_by_id(&*db, id).await?;
    let line_size = get_from_app_store::<usize>(&app_handle, AppStoreKey::LineSize)?.unwrap_or(50);

    set_to_app_store(&app_handle, AppStoreKey::LastReadNovelId, novel.id)?;

    // 创建 reader 并更新状态
    let reader = NovelReader::new(
        novel.id,
        novel.path,
        novel.read_position as usize,
        line_size,
    )?;

    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.novel_reader = Some(reader);

    app_handle.emit("reader-change", 0).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn delete_novel(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let novel = get_novel_by_id(&*db, id).await?;

    let filepath = Path::new(&novel.path);

    if filepath.exists() {
        remove_file(filepath).map_err(|e| format!("Error deleting file: {}", e))?;
    }

    delete_novel_from_db(&*db, id).await?;

    // 如果删除的是正在阅读的小说，则关闭对应阅读器，并重置上次阅读的小说
    let mut state = state.lock().map_err(|e| e.to_string())?;

    if let Some(reader) = &state.novel_reader {
        if reader.novel_id == id {
            state.novel_reader = None;
            delete_from_app_store(&app_handle, AppStoreKey::LastReadNovelId)?;
        }
    }

    app_handle.emit("reader-change", 0).unwrap();

    Ok(())
}
