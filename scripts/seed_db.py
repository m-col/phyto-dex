#!/usr/bin/env python3
#
# Seed the database with data from World Flora Online. This will fetch data from their
# graphql endpoint containing information on:
#
#   - Araceae family
#   - Every genus within the family
#   - Every species within the Monstera genus (which is within Aracaeae)
#
# Some other bits of graphql queries are kept here too for reference and/or future use.
#
# NB: this is a rough script, don't expect production code

import requests
import sqlite3
from typing import Any

_ENDPOINT = "https://list.worldfloraonline.org/gql.php"

ARACEAE = "wfo-7000000042"


def _make_request(query: str, **variables: str) -> dict[str, Any]:
    """Send the query to the graphql endpoint and print the response."""
    payload = {
        "query": query,
        "variables": variables,
    }

    response = requests.post(_ENDPOINT, json=payload)
    data = response.json()
    return data


def fetch_wfo(query_string: str) -> dict[str, Any]:
    query = """
    query NameSearch($terms: String!){
        taxonNameSuggestion(
            termsString: $terms
            limit: 100
        ) {
            id
            stableUri
            fullNameStringPlain,
            currentPreferredUsage{
                hasName{
                    id,
                    stableUri,
                    fullNameStringPlain
                }
            }
        }
    }
    """
    return _make_request(query, terms=query_string)


def query_types() -> dict[str, Any]:
    query = """
    query {
      __schema {
        types {
          name
          kind
          description
          fields {
            name
          }
        }
      }
    }
    """
    return _make_request(query)


def query_query_types() -> dict[str, Any]:
    query = """
    query {
      __type(name: "TaxonName") {
        fields {
          name
          description
          args {
            name
            description
            type {
              name
              kind
            }
          }
          type {
            name
            kind
            ofType {
              name
              kind
            }
          }
        }
      }
    }
    """
    return _make_request(query)


def taxonNameById(name_id=ARACEAE) -> dict[str, Any]:
    query = """
    query TaxonInfo($name_id: String!){
        taxonNameById(nameId: $name_id){
            id
            title
            fullNameStringPlain
            fullNameStringNoAuthorsPlain
            nameString
            genusString
            rank
            wfoPath
            currentPreferredUsage {
                hasName {
                    id
                    fullNameStringPlain
                    fullNameStringNoAuthorsPlain
                    nameString
                }
            }
        }
    }
    """
    return _make_request(query, name_id=name_id)


def get_children_of_taxon(name_id=ARACEAE) -> dict[str, Any]:
    query = """
    query GenusList($name_id: String!){
        taxonNameById(nameId: $name_id){
            id
            currentPreferredUsage {
                hasPart {
                    id
                    hasName {
                        id
                        fullNameStringNoAuthorsPlain
                        nameString
                    }
                }
            }
        }
    }
    """
    # hasSynonym {
    #    id
    #    fullNameStringNoAuthorsPlain
    #    nameString
    # }
    return _make_request(query, name_id=name_id)


def get_seed_data(db_path: str = "storage.db") -> None:
    """The main function."""
    # Get info on the family name
    family = taxonNameById(ARACEAE)["data"]["taxonNameById"]
    family_name = family["fullNameStringNoAuthorsPlain"]
    # Confirm this family name is the one in usage (not a synonym)
    assert (
        family["currentPreferredUsage"]["hasName"]["fullNameStringNoAuthorsPlain"]
        == family_name
    )
    wfo_id = family["currentPreferredUsage"]["hasName"]["id"]
    # This is actually the same as we have hard-coded, but let's check here for
    # demonstration purposes
    assert wfo_id == ARACEAE

    # Get data for genera in this family
    data = get_children_of_taxon(ARACEAE)["data"]["taxonNameById"]
    assert data["id"] == ARACEAE
    genera = []
    monstera: str | None = None
    genus_data = data["currentPreferredUsage"]["hasPart"]
    for genus in genus_data:
        name = genus["hasName"]["fullNameStringNoAuthorsPlain"]
        wfo_id = genus["hasName"]["id"]
        genera.append((name, wfo_id))

        if monstera is None and name == "Monstera":
            monstera = wfo_id

    assert monstera is not None, "Could not find Monstera genus"

    # Get data for species in the Monstera genus
    data = get_children_of_taxon(monstera)["data"]["taxonNameById"]
    assert data["id"] == monstera
    species_row = []
    species_data = data["currentPreferredUsage"]["hasPart"]
    for species in species_data:
        name = species["hasName"]["fullNameStringNoAuthorsPlain"]
        wfo_id = species["hasName"]["id"]
        species_row.append((name, wfo_id))

    # Finally, add this data into the database
    with sqlite3.connect(db_path) as conn:
        c = conn.cursor()
        c.execute(
            "INSERT INTO family (name, wfo_id) VALUES (?, ?) RETURNING id",
            (family_name, wfo_id),
        )
        (family_id,) = c.fetchone()

        for name, wfo_id in genera:
            c.execute(
                "INSERT INTO genus (name, wfo_id, family) VALUES (?, ?, ?)",
                (name, wfo_id, family_id),
            )

        c.execute("SELECT id FROM genus WHERE wfo_id = ?", (monstera,))
        (genus_id,) = c.fetchone()

        for name, wfo_id in species_row:
            c.execute(
                "INSERT INTO species (name, wfo_id, genus) VALUES (?, ?, ?)",
                (name, wfo_id, genus_id),
            )


if __name__ == "__main__":
    get_seed_data()
