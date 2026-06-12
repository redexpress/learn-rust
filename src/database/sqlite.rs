use rusqlite::{Connection, Result, params};

use crate::cli::SqliteCmd;

pub(crate) fn run(cmd: SqliteCmd) -> Result<()> {
    match cmd {
        SqliteCmd::Demo => demo(),
    }
}

fn demo() -> Result<()> {
    let conn = Connection::open("demo.db")?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS demo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            price REAL,
            create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            image BLOB
        );"
    )?;

    let image_data: Option<Vec<u8>> = Some(vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10]);

    conn.execute(
        "INSERT INTO demo (name, price, image) VALUES (?1, ?2, ?3)",
        params!["Apple", 3.5, image_data],
    )?;

    conn.execute(
        "INSERT INTO demo (name, price, image) VALUES (?1, ?2, ?3)",
        params!["Banana", 2.0, None::<Vec<u8>>],
    )?;

    let id: i64 = conn.query_row(
        "SELECT id FROM demo WHERE name = ?1",
        params!["Apple"],
        |row| row.get(0),
    )?;
    println!("Apple id = {}", id);

    conn.execute(
        "UPDATE demo SET price = ?1 WHERE id = ?2",
        params![3.99, id],
    )?;

    let mut stmt = conn.prepare("SELECT id, name, price, create_at FROM demo")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, Option<f64>>(2)?,
            row.get::<_, Option<String>>(3)?,
        ))
    })?;

    for row in rows {
        let (id, name, price, create_at) = row?;
        println!("id={}, name={:?}, price={:?}, create_at={:?}", id, name, price, create_at);
    }

    let image: Option<Vec<u8>> = conn.query_row(
        "SELECT image FROM demo WHERE id = ?1",
        params![id],
        |row| row.get(0),
    )?;
    println!("image bytes = {:?}", image.as_ref().map(|v| v.len()));

    conn.execute("DELETE FROM demo WHERE id = ?1", params![id])?;
    println!("Deleted id = {}", id);

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM demo", [], |row| row.get(0))?;
    println!("remaining rows = {}", count);

    Ok(())
}
