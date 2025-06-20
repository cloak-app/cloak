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
    let reader = &state.novel_reader;

    if let Some(reader) = reader {
        Ok(reader.clone())
    } else {
        Err("No novel is currently open".to_string())
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
        .emit("reader-line-num-changed", 0)
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
        Err("No novel is currently open".to_string())
    }
}

#[tauri::command]
pub async fn set_line_num(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    line_num: usize,
) -> Result<(), String> {
    let (novel_id, last_read_position, read_progress);
    {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.set_line_num(line_num)?;
            app_handle
                .emit("reader-line-num-changed", 0)
                .map_err(|e| e.to_string())?;
            novel_id = reader.novel.id;
            last_read_position = reader.line_num as i64;
            read_progress = reader.read_progress();
        } else {
            return Err("No novel is currently open".to_string());
        }
    }

    save_novel(&*db, novel_id, last_read_position, read_progress).await?;

    Ok(())
}

#[tauri::command]
pub async fn next_line(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, last_read_position, read_progress);
    {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.next_line()?;
            app_handle
                .emit("reader-line-num-changed", 0)
                .map_err(|e| e.to_string())?;
            novel_id = reader.novel.id;
            last_read_position = reader.line_num as i64;
            read_progress = reader.read_progress();
        } else {
            return Err("No novel is currently open".to_string());
        }
    }

    save_novel(&*db, novel_id, last_read_position, read_progress).await?;

    Ok(())
}

#[tauri::command]
pub async fn prev_line(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, last_read_position, read_progress);
    {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.prev_line()?;
            app_handle
                .emit("reader-line-num-changed", 0)
                .map_err(|e| e.to_string())?;
            novel_id = reader.novel.id;
            last_read_position = reader.line_num as i64;
            read_progress = reader.read_progress();
        } else {
            return Err("No novel is currently open".to_string());
        }
    }

    save_novel(&*db, novel_id, last_read_position, read_progress).await?;

    Ok(())
}

#[tauri::command]
pub async fn next_chapter(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, last_read_position, read_progress);
    {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.next_chapter()?;
            app_handle
                .emit("reader-line-num-changed", 0)
                .map_err(|e| e.to_string())?;
            novel_id = reader.novel.id;
            last_read_position = reader.line_num as i64;
            read_progress = reader.read_progress();
        } else {
            return Err("No novel is currently open".to_string());
        }
    }

    save_novel(&*db, novel_id, last_read_position, read_progress).await?;

    Ok(())
}

#[tauri::command]
pub async fn prev_chapter(
    app_handle: tauri::AppHandle,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let (novel_id, last_read_position, read_progress);
    {
        let mut state = state.lock().map_err(|e| e.to_string())?;
        let reader = &mut state.novel_reader;

        if let Some(reader) = reader {
            reader.prev_chapter()?;
            app_handle
                .emit("reader-line-num-changed", 0)
                .map_err(|e| e.to_string())?;
            novel_id = reader.novel.id;
            last_read_position = reader.line_num as i64;
            read_progress = reader.read_progress();
        } else {
            return Err("No novel is currently open".to_string());
        }
    }

    save_novel(&*db, novel_id, last_read_position, read_progress).await?;

    Ok(())
}
