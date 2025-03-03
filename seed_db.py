#!/usr/bin/env python3
#
# Seed the database with some testing data.

import sqlite3
from pathlib import Path


def seed_db(db_path: Path):
    """Seed the database with some testing data."""

    with sqlite3.connect(db_path) as conn:
        c = conn.cursor()

        # Add a family
        c.execute("INSERT INTO family (name) VALUES ('Araceae')")

        # Add a genus to that family
        c.execute("INSERT INTO genus (name, family) VALUES ('Monstera', (SELECT id FROM family WHERE name = 'Araceae'))")

        # Add some species to that genus
        c.execute("INSERT INTO species (name, genus) VALUES ('Monstera glaucescens', (SELECT id FROM genus WHERE name = 'Monstera'))")
        c.execute("INSERT INTO species (name, genus) VALUES ('Monstera punctulata', (SELECT id FROM genus WHERE name = 'Monstera'))")

        print("Added data!")


if __name__ == "__main__":
    seed_db(Path("storage.db"))
