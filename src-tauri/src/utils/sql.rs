use crate::db::{model::Novel, Db};

// 新增小说
pub async fn add_novel(
    db: &Db,
    title: &str,
    cover: Option<Vec<u8>>,
    author: Option<String>,
    description: Option<String>,
    path: &str,
    file_size: i64,
) -> Result<(), String> {
    let sql = r#"
        INSERT INTO novel (
            title, cover, author, description, path, read_position, read_progress, file_size
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
    "#;
    sqlx::query(sql)
        .bind(title)
        .bind(cover)
        .bind(author)
        .bind(description)
        .bind(path)
        .bind(0)
        .bind(0)
        .bind(file_size)
        .execute(db)
        .await
        .map_err(|e| format!("添加小说失败: {}", e))?;

    Ok(())
}

// 根据 id 获取小说
pub async fn get_novel_by_id(db: &Db, id: i64) -> Result<Novel, String> {
    let novel = sqlx::query_as::<_, Novel>("SELECT * FROM novel WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| format!("获取小说失败: {}", e))?;

    Ok(novel)
}

// 获取小说列表
pub async fn get_novel_list(db: &Db) -> Result<Vec<Novel>, String> {
    let novels = sqlx::query_as::<_, Novel>("SELECT * FROM novel")
        .fetch_all(db)
        .await
        .map_err(|e| format!("获取小说列表失败: {}", e))?;

    Ok(novels)
}

// 保存小说
pub async fn save_novel(
    db: &Db,
    novel_id: i64,
    read_position: i64,
    read_progress: f64,
) -> Result<(), String> {
    sqlx::query("UPDATE novel SET read_position = ?, read_progress = ? WHERE id = ?")
        .bind(read_position)
        .bind(read_progress)
        .bind(novel_id)
        .execute(db)
        .await
        .map_err(|e| format!("保存小说失败: {}", e))?;

    Ok(())
}

// 根据 id 删除小说
pub async fn delete_novel(db: &Db, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM novel WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("删除小说失败: {}", e))?;

    Ok(())
}

// 打开小说
pub async fn open_novel(db: &Db, id: i64) -> Result<(), String> {
    sqlx::query("UPDATE novel SET is_open = 1 WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("打开小说失败: {}", e))?;

    Ok(())
}

// 关闭小说
pub async fn close_novel(db: &Db, id: i64) -> Result<(), String> {
    sqlx::query("UPDATE novel SET is_open = 0 WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("关闭小说失败: {}", e))?;

    Ok(())
}

// 获取正在阅读的小说
pub async fn get_open_novel(db: &Db) -> Result<Novel, String> {
    let novel = sqlx::query_as::<_, Novel>("SELECT * FROM novel WHERE is_open = 1")
        .fetch_one(db)
        .await
        .map_err(|e| format!("获取正在阅读的小说失败: {}", e))?;

    Ok(novel)
}
