use std::sync::Mutex;

use tauri::Emitter;

use crate::{
    constants::event::*,
    db::Db,
    state::model::AppState,
    utils::{reader::NovelReader, sql},
};

#[tauri::command]
pub async fn get_novel_reader(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<NovelReader, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let reader = &state.novel_reader;

    if let Some(reader) = reader {
        Ok(reader.clone())
    } else {
        Err("暂无打开的小说".to_string())
    }
}

#[tauri::command]
pub async fn close_novel_reader(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    state.novel_reader = None;

    app_handle
        .emit(READER_CHANGE, ())
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_line(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    let reader = &state.novel_reader;

    if let Some(reader) = reader {
        let line = reader.get_line().unwrap_or("");
        Ok(line.to_string())
    } else {
        Err("暂无打开的小说".to_string())
    }
}

#[tauri::command]
pub async fn set_read_position(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    read_position: usize,
) -> Result<(), String> {
    let (novel_id, read_position, read_progress) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.set_read_position(read_position)?;
            app_handle
                .emit(READER_CHANGE, ())
                .map_err(|e| e.to_string())?;
            (
                reader.novel_id,
                reader.read_position as i64,
                reader.read_progress(),
            )
        } else {
            return Err("暂无打开的小说".to_string());
        }
    };

    sql::save_novel(&db, novel_id, read_position, read_progress).await?;

    Ok(())
}

#[tauri::command]
pub async fn next_line(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, read_position, read_progress) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.next_line()?;
            app_handle
                .emit(READER_CHANGE, ())
                .map_err(|e| e.to_string())?;
            (
                reader.novel_id,
                reader.read_position as i64,
                reader.read_progress(),
            )
        } else {
            return Err("暂无打开的小说".to_string());
        }
    };

    sql::save_novel(&db, novel_id, read_position, read_progress).await?;

    Ok(())
}

#[tauri::command]
pub async fn prev_line(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, read_position, read_progress) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.prev_line()?;
            app_handle
                .emit(READER_CHANGE, ())
                .map_err(|e| e.to_string())?;
            (
                reader.novel_id,
                reader.read_position as i64,
                reader.read_progress(),
            )
        } else {
            return Err("暂无打开的小说".to_string());
        }
    };

    sql::save_novel(&db, novel_id, read_position, read_progress).await?;

    Ok(())
}

#[tauri::command]
pub async fn next_chapter(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, read_position, read_progress) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.next_chapter()?;
            app_handle
                .emit(READER_CHANGE, ())
                .map_err(|e| e.to_string())?;
            (
                reader.novel_id,
                reader.read_position as i64,
                reader.read_progress(),
            )
        } else {
            return Err("暂无打开的小说".to_string());
        }
    };

    sql::save_novel(&db, novel_id, read_position, read_progress).await?;

    Ok(())
}

#[tauri::command]
pub async fn prev_chapter(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, read_position, read_progress) = {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.prev_chapter()?;
            app_handle
                .emit(READER_CHANGE, ())
                .map_err(|e| e.to_string())?;
            (
                reader.novel_id,
                reader.read_position as i64,
                reader.read_progress(),
            )
        } else {
            return Err("暂无打开的小说".to_string());
        }
    };

    sql::save_novel(&db, novel_id, read_position, read_progress).await?;

    Ok(())
}
