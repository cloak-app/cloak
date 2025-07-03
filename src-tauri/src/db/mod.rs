use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tauri::{App, Manager};

pub mod model;

pub type Db = Pool<Sqlite>;

pub async fn setup_db(app: &App) -> Db {
    let mut path = app.path().app_data_dir().expect("获取目录失败");

    std::fs::create_dir_all(path.clone()).expect("创建目录失败");

    path.push("db.sqlite");

    Sqlite::create_database(format!("sqlite:{}", path.to_str().unwrap()).as_str())
        .await
        .expect("创建数据库失败");

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .expect("连接数据库失败");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("数据库迁移失败");

    db
}
