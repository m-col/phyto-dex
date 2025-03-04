use dioxus::logger::tracing::warn;
use dioxus::prelude::*;

use crate::data::*;

#[cfg(feature = "server")]
thread_local! {
    static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("storage.db").expect("Failed to open database");

        match conn.execute_batch("
            CREATE TABLE IF NOT EXISTS family (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                wfo_id TEXT NOT NULL UNIQUE
            );
            CREATE TABLE IF NOT EXISTS genus (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                wfo_id TEXT NOT NULL UNIQUE,
                family INTEGER,
                FOREIGN KEY (family) REFERENCES family (id)
            );
            CREATE TABLE IF NOT EXISTS species (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                wfo_id TEXT NOT NULL UNIQUE,
                genus INTEGER,
                FOREIGN KEY (genus) REFERENCES genus(id)
            );
            CREATE TABLE IF NOT EXISTS specimen (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                species INTEGER,
                FOREIGN KEY (species) REFERENCES species(id)
            );

            ",
        ) {
            Ok(_) => conn,
            Err(e) => {
                warn!("Failed to create database: {:?}", e);
                panic!("Failed to create database: {:?}", e);
            }
        }
    };
}

#[server]
pub async fn add_specimen(name: String, species: SpeciesId) -> Result<SpecimenId, ServerFnError> {
    DB.with(|f| {
        f.execute(
            "INSERT INTO specimen (name, species) VALUES (?1, ?2)",
            (&name, species),
        )
    })?;

    let specimen_id = DB.with(|f| {
        f.prepare("SELECT id FROM specimen WHERE name = ?")
            .unwrap()
            .query_row([&name], |row| row.get(0))
            .unwrap()
    });

    Ok(specimen_id)
}

#[server]
pub async fn list_species() -> Result<Vec<Species>, ServerFnError> {
    let species = DB.with(|f| {
        f.prepare("SELECT id, name, genus FROM species")
            .unwrap()
            .query_map([], |row| {
                Ok(Species {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    genus: row.get(2)?,
                })
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(species)
}

#[server]
pub async fn get_collection() -> Result<Collection, ServerFnError> {
    let specimens = DB.with(|f| {
        f.prepare("SELECT id, name, species FROM specimen")
            .unwrap()
            .query_map([], |row| {
                Ok(Specimen {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    species_id: row.get(2)?,
                })
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    let species = DB.with(|f| {
        f.prepare(
            "
            SELECT species.id, species.name, species.genus
            FROM species
            INNER JOIN specimen ON species.id = specimen.species
        ",
        )
        .unwrap()
        .query_map([], |row| {
            let species_id = row.get(0)?;
            Ok((
                species_id,
                Species {
                    id: species_id,
                    name: row.get(1)?,
                    genus: row.get(2)?,
                },
            ))
        })
        .unwrap()
        .map(|r| r.unwrap())
        .collect()
    });

    Ok(Collection {
        specimens: specimens,
        species: species,
    })
}

#[server]
pub async fn get_species_by_id(species_id: SpeciesId) -> Result<Species, ServerFnError> {
    let species = DB.with(|f| {
        f.prepare("SELECT name, genus FROM species WHERE id = ?")
            .unwrap()
            .query_row([species_id], |row| {
                Ok(Species {
                    id: species_id,
                    name: row.get(0)?,
                    genus: row.get(1)?,
                })
            })
            .unwrap()
    });

    Ok(species)
}
