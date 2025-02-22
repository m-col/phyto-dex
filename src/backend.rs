use dioxus::logger::tracing::warn;
use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("storage.db").expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS specimen (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                species INTEGER,
                FOREIGN KEY (species) REFERENCES species(id)
            );
            CREATE TABLE IF NOT EXISTS species (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            );
            INSERT INTO species (name) VALUES ('Monstera deliciosa');
            INSERT INTO species (name) VALUES ('Monstera adansonii');
            ",
        ).unwrap();

        conn
    };
}

#[server]
pub async fn add_specimen(name: String, species: i32) -> Result<(), ServerFnError> {
    DB.with(|f| {
        f.execute(
            "INSERT INTO specimen (name, species) VALUES (?1, ?2)",
            (&name, species),
        )
    })?;
    warn!("Species: {:?}", list_species().await);
    warn!("Spcimens: {:?}", list_specimens().await);
    Ok(())
}

#[server]
pub async fn list_species() -> Result<Vec<(i32, String)>, ServerFnError> {
    let species = DB.with(|f| {
        f.prepare("SELECT id, name FROM species")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(species)
}

#[server]
pub async fn list_specimens() -> Result<Vec<(i32, String, i32)>, ServerFnError> {
    let specimens = DB.with(|f| {
        f.prepare("SELECT id, name, species FROM specimen")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(specimens)
}
