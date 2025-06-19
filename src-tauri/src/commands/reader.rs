use crate::db::Db;
use crate::state::model::AppState;
use crate::utils::novel::save_novel;
use crate::utils::reader::NovelReader;
use std::sync::Mutex;
use tauri::Emitter;

#[tauri::command]
pub async fn get_novel_reader(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<NovelReader, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let reader = state
        .novel_reader
        .as_ref()
        .ok_or("No novel is currently open")?;

    Ok(reader.clone())
}

#[tauri::command]
pub async fn get_line(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let reader = state
        .novel_reader
        .as_mut()
        .ok_or("No novel is currently open")?;

    let line = reader.get_line();

    Ok(line.unwrap_or("").to_string())
}

#[tauri::command]
pub async fn set_line_num(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    line_num: usize,
) -> Result<(), String> {
    let novel_id = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = state
            .novel_reader
            .as_mut()
            .ok_or("No novel is currently open")?;

        reader.set_line_num(line_num)?;
        reader.novel.id
    };

    app_handle.emit("reader-line-num-changed", 0).unwrap();

    save_novel(&*db, novel_id, line_num as i64).await?;

    Ok(())
}

#[tauri::command]
pub async fn next_line(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, line_num) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = state
            .novel_reader
            .as_mut()
            .ok_or("No novel is currently open")?;

        reader.next_line()?;
        (reader.novel.id, reader.line_num)
    };

    app_handle.emit("reader-line-num-changed", 0).unwrap();

    save_novel(&*db, novel_id, line_num as i64).await?;

    Ok(())
}

#[tauri::command]
pub async fn prev_line(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, line_num) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = state
            .novel_reader
            .as_mut()
            .ok_or("No novel is currently open")?;

        reader.prev_line()?;
        (reader.novel.id, reader.line_num)
    };

    app_handle.emit("reader-line-num-changed", 0).unwrap();

    save_novel(&*db, novel_id, line_num as i64).await?;

    Ok(())
}

#[tauri::command]
pub async fn next_chapter(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, line_num) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = state
            .novel_reader
            .as_mut()
            .ok_or("No novel is currently open")?;

        reader.next_chapter()?;
        (reader.novel.id, reader.line_num)
    };

    app_handle.emit("reader-line-num-changed", 0).unwrap();

    save_novel(&*db, novel_id, line_num as i64).await?;

    Ok(())
}

#[tauri::command]
pub async fn prev_chapter(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, line_num) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = state
            .novel_reader
            .as_mut()
            .ok_or("No novel is currently open")?;

        reader.prev_chapter()?;
        (reader.novel.id, reader.line_num)
    };

    app_handle.emit("reader-line-num-changed", 0).unwrap();

    save_novel(&*db, novel_id, line_num as i64).await?;

    Ok(())
}
