use crate::db::model::Novel;
use crate::db::Db;
use crate::reader::Chapter;
use crate::reader::NovelReader;
use crate::state::AppState;
use std::sync::Mutex;
use tauri::Runtime;
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn add_novel(db: tauri::State<'_, Db>, path: &str) -> Result<(), String> {
    let filename = path.split("/").last().unwrap();

    // 校验文件后缀名为 .txt
    if !filename.ends_with(".txt") {
        return Err("File must have a `.txt` extension".to_string());
    }

    let title = filename.split(".").next().unwrap();

    sqlx::query("INSERT INTO novel (title, path, last_read_position) VALUES (?1, ?2, ?3)")
        .bind(title)
        .bind(path)
        .bind(0)
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

pub async fn get_novel_by_id(db: &Db, id: i64) -> Result<Novel, String> {
    let novel = sqlx::query_as::<_, Novel>("SELECT * FROM novel WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| format!("Error fetching novel: {}", e))?;

    Ok(novel)
}

#[tauri::command]
pub async fn open_novel<R: Runtime>(
    app: tauri::AppHandle<R>,
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let novel = get_novel_by_id(&*db, id).await?;

    let store = app.store("app_data.json").unwrap();

    store.set("last_read_novel_id", novel.id);

    // 创建 reader 并更新状态
    let reader = NovelReader::new(novel);

    if let Ok(reader) = reader {
        let mut state = state.lock().unwrap();
        state.novel_reader = Some(reader);
        Ok(())
    } else {
        Err(format!("Failed to open novel!"))
    }
}

#[tauri::command]
pub async fn get_current_novel(state: tauri::State<'_, Mutex<AppState>>) -> Result<Novel, String> {
    let state = state.lock().unwrap();
    let reader = state
        .novel_reader
        .as_ref()
        .ok_or("No novel is currently open")?;

    Ok(reader.novel.clone())
}

#[tauri::command]
pub async fn get_chapter_list(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Chapter>, String> {
    let state = state.lock().unwrap();
    let reader = state
        .novel_reader
        .as_ref()
        .ok_or("No novel is currently open")?;
    Ok(reader.chapters.clone())
}

#[tauri::command]
pub async fn get_line(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, String> {
    let mut state = state.lock().unwrap();
    let reader = state
        .novel_reader
        .as_mut()
        .ok_or("No novel is currently open")?;

    let line = reader.get_line();

    Ok(line.unwrap_or("").to_string())
}

#[tauri::command]
pub async fn next_line(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, String> {
    let mut state = state.lock().unwrap();
    let reader = state
        .novel_reader
        .as_mut()
        .ok_or("No novel is currently open")?;

    reader.set_line_num(reader.line_num + 1)?;
    let line = reader.get_line();

    Ok(line.unwrap_or("").to_string())
}

#[tauri::command]
pub async fn prev_line(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, String> {
    let mut state = state.lock().unwrap();
    let reader = state
        .novel_reader
        .as_mut()
        .ok_or("No novel is currently open")?;

    reader.set_line_num(reader.line_num - 1)?;
    let line = reader.get_line();

    Ok(line.unwrap_or("").to_string())
}
