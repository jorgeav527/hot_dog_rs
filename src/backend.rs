use dioxus::prelude::*;

#[post("/api/save_dog")]
pub async fn save_dog(image: String) -> Result<()> {
    use std::io::Write;

    // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    let _ = file.write_fmt(format_args!("{image}\n"));
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

// The database is only available to server code
#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        // Return the connection
        conn
    };
}

#[server]
pub async fn list_dogs() -> Result<Vec<(i64, String)>, ServerFnError> {
    // Change usize to i64
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            }) // Explicit types help
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}
