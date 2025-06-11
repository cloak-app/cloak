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

pub async fn save_novel(db: &Db, novel_id: i64, last_read_position: i64) -> Result<(), String> {
    sqlx::query("UPDATE novel SET last_read_position = ? WHERE id = ?")
        .bind(last_read_position)
        .bind(novel_id)
        .execute(db)
        .await
        .map_err(|e| format!("Error saving novel: {}", e))?;

    Ok(())
}
