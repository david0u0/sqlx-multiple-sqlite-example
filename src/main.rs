const DB1: &str = "output/db1.db";
const DB2: &str = "output/db2.db";
const SCHEMA: &str = "schema.db";

use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

async fn get_pool(file: &str) -> SqlitePool {
    SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename(file)
            .create_if_missing(true),
    )
    .await
    .unwrap()
}

async fn insert_stuff(pool: &SqlitePool) {
    for i in 0..5 {
        let name = i.to_string();
        sqlx::query!("INSERT INTO my_data (name) VALUES(?) RETURNING id", name)
            .fetch_one(pool)
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    std::fs::copy(SCHEMA, DB1).unwrap();
    std::fs::copy(SCHEMA, DB2).unwrap();

    let pool1 = get_pool(DB1).await;
    let pool2 = get_pool(DB2).await;

    insert_stuff(&pool1).await;
    insert_stuff(&pool2).await;

    pool1.close().await;
    pool2.close().await;
}
