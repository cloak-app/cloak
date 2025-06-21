use crate::db::model::Novel;
use crate::db::Db;

pub async fn get_novel_by_id(db: &Db, id: i64) -> Result<Novel, String> {
    let novel = sqlx::query_as::<_, Novel>("SELECT * FROM novel WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|e| format!("Error fetching novel: {}", e))?;

    Ok(novel)
}

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
        .map_err(|e| format!("Error saving novel: {}", e))?;

    Ok(())
}

pub async fn delete_novel_from_db(db: &Db, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM novel WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("Error deleting novel: {}", e))?;

    Ok(())
}
