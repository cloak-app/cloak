use crate::db::model::Novel;
use crate::db::Db;
use crate::reader::NovelReader;
use crate::state::AppState;
use std::sync::Mutex;

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
pub async fn open_novel(
    db: tauri::State<'_, Db>,
    state: tauri::State<'_, Mutex<AppState>>,
    id: i64,
) -> Result<(), String> {
    let novel = get_novel_by_id(&*db, id).await?;

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
