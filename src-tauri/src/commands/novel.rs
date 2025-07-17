use std::{
    fs,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    sync::Mutex,
};

use charset_normalizer_rs::from_bytes;
use epub::doc::EpubDoc;
use tauri::{Emitter, Manager};

use crate::{
    constants::event::*,
    db::{model::Novel, Db},
    state::model::AppState,
    store::{get_from_app_store, model::AppStoreKey},
    utils::{reader::NovelReader, sql},
};

#[tauri::command]
pub async fn add_novel(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    path: &str,
) -> Result<(), String> {
    let filepath = Path::new(path);

    // 校验文件是否存在
    if !filepath.exists() {
        return Err(format!("文件不存在: {path}"));
    }

    let filename = filepath
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("无法获取文件名: {path}"))?;

    let extension = filepath
        .extension()
        .ok_or_else(|| format!("无法获取文件扩展名: {path}"))?;

    // 校验文件后缀名为 .txt 或 .epub
    if extension != "txt" && extension != "epub" {
        return Err(format!("文件格式不支持: {filename}"));
    }

    // 检查文件编码格式是否受支持
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut buf_reader = BufReader::new(file);
    let mut buffer = Vec::new();

    buf_reader
        .read_to_end(&mut buffer)
        .map_err(|e| e.to_string())?;

    let result = from_bytes(&buffer, None);
    result.get_best().ok_or("文件编码不支持")?;

    // 将文件复制到应用程序目录
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let new_path = app_data_dir.join(filename);

    fs::copy(path, &new_path).map_err(|e| e.to_string())?;

    let title = match filename.split(".").next() {
        Some(v) => v,
        None => return Err(format!("非法文件名: {filename}")),
    };

    let (cover, author, description) = if extension == "epub" {
        let mut doc = EpubDoc::new(&new_path).map_err(|e| e.to_string())?;
        let cover = doc.get_cover().map(|(blob, _)| blob);
        let author = doc.mdata("creator");
        let description = doc.mdata("description");
        (cover, author, description)
    } else {
        (None, None, None)
    };

    let new_path_str = new_path
        .to_str()
        .ok_or_else(|| format!("文件路径转换失败: {new_path:?}"))?;

    let file_size = filepath
        .metadata()
        .map(|metadata| metadata.len())
        .map_err(|e| e.to_string())?;

    sql::add_novel(
        &db,
        title,
        cover,
        author,
        description,
        new_path_str,
        file_size as i64,
    )
    .await?;

    Ok(())
}

#[tauri::command]
pub async fn get_novel_list(db: tauri::State<'_, Db>) -> Result<Vec<Novel>, String> {
    let novels = sql::get_novel_list(&db).await?;

    Ok(novels)
}

#[tauri::command]
pub async fn get_novel_detail(db: tauri::State<'_, Db>, id: i64) -> Result<Novel, String> {
    let novel = sql::get_novel_by_id(&db, id).await?;

    Ok(novel)
}

#[tauri::command]
pub async fn open_novel(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let novel = sql::get_novel_by_id(&db, id).await?;

    {
        let line_size = get_from_app_store::<usize>(&app_handle, AppStoreKey::LineSize).unwrap();

        // 创建 reader 并更新状态
        let reader = NovelReader::new(
            novel.id,
            novel.path,
            novel.read_position as usize,
            line_size,
        )?;
        let mut state = state.lock().map_err(|e| e.to_string())?;
        state.novel_reader = Some(reader);
    }

    sql::open_novel(&db, id).await?;

    app_handle.emit(READER_CHANGE, ()).unwrap();

    Ok(())
}

#[tauri::command]
pub async fn close_novel(
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    sql::close_novel(&db, id).await?;

    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.novel_reader = None;

    Ok(())
}

#[tauri::command]
pub async fn delete_novel(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let novel = sql::get_novel_by_id(&db, id).await?;

    let filepath = Path::new(&novel.path);

    if filepath.exists() {
        fs::remove_file(filepath).map_err(|e| e.to_string())?;
    }

    sql::delete_novel(&db, id).await?;

    let mut state = state.lock().map_err(|e| e.to_string())?;

    if let Some(reader) = &state.novel_reader {
        if reader.novel_id == id {
            state.novel_reader = None;
        }
    }

    app_handle.emit(READER_CHANGE, ()).unwrap();

    Ok(())
}
