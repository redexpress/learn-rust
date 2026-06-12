use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Row, SqlitePool};

use crate::cli::SqlxCmd;

pub(crate) async fn run(cmd: SqlxCmd) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        SqlxCmd::Demo => demo().await,
    }
}

async fn demo() -> Result<(), Box<dyn std::error::Error>> {
    let pool: SqlitePool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite:./demo_sqlx.db?mode=rwc")
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS demo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            price REAL,
            create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            image BLOB
        );"
    )
    .execute(&pool)
    .await?;

    let image_data: Option<Vec<u8>> = Some(vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10]);
    let now = chrono::Utc::now();

    sqlx::query("INSERT INTO demo (name, price, image, create_at) VALUES (?1, ?2, ?3, ?4)")
        .bind("Apple")
        .bind(3.5_f64)
        .bind(image_data)
        .bind(now)
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO demo (name, price, image) VALUES (?1, ?2, ?3)")
        .bind("Banana")
        .bind(2.0_f64)
        .bind(None::<Vec<u8>>)
        .execute(&pool)
        .await?;

    let id: i64 = sqlx::query_scalar("SELECT id FROM demo WHERE name = ?1")
        .bind("Apple")
        .fetch_one(&pool)
        .await?;
    println!("Apple id = {}", id);

    sqlx::query("UPDATE demo SET price = ?1 WHERE id = ?2")
        .bind(3.99_f64)
        .bind(id)
        .execute(&pool)
        .await?;

    let rows = sqlx::query("SELECT id, name, price, create_at FROM demo")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let id: i64 = row.try_get("id")?;
        let name: Option<String> = row.try_get("name")?;
        let price: Option<f64> = row.try_get("price")?;
        let create_at: Option<chrono::NaiveDateTime> = row.try_get("create_at")?;
        println!("id={}, name={:?}, price={:?}, create_at={:?}", id, name, price, create_at);
    }

    let image: Option<Vec<u8>> = sqlx::query_scalar("SELECT image FROM demo WHERE id = ?1")
        .bind(id)
        .fetch_one(&pool)
        .await?;
    println!("image bytes = {:?}", image.as_ref().map(|v| v.len()));

    sqlx::query("DELETE FROM demo WHERE id = ?1")
        .bind(id)
        .execute(&pool)
        .await?;
    println!("Deleted id = {}", id);

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM demo")
        .fetch_one(&pool)
        .await?;
    println!("remaining rows = {}", count);

    pool.close().await;
    Ok(())
}
