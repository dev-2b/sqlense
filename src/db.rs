use crate::models::{ColumnInfo, TableInfo};
use anyhow::Result as AnyResult;
use rusqlite::Connection;
pub fn establish_connection() -> AnyResult<Connection> {
    let conn = Connection::open("test_db/dummy.db")?;
    Ok(conn)
}

pub fn get_tables(conn: &Connection) -> AnyResult<Vec<TableInfo>> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%';",
    )?;
    let table_name_iter = stmt.query_map([], |row| {
        let name: String = row.get(0)?;

        Ok(name)
    })?;
    let mut tables = Vec::new();
    for name_result in table_name_iter {
        let table_name = name_result?;

        // Hier rufen wir unsere neue Hilfsfunktion auf!
        let columns = get_columns(conn, &table_name)?;

        // Wir bauen das fertige DTO zusammen
        tables.push(TableInfo {
            name: table_name,
            columns,
        });
    }

    Ok(tables)
}

// Hilfsfunktion: Holt alle Spalten für einen bestimmten Tabellennamen
pub fn get_columns(conn: &Connection, table_name: &str) -> AnyResult<Vec<ColumnInfo>> {
    // Da PRAGMA keine Platzhalter erlaubt, bauen wir den String mit format! um dynamisch den Tabellennamen einzufügen. Wir müssen hier aufpassen, dass der Tabellenname nicht von außen kommt, sonst hätten wir ein SQL-Injection-Problem.
    let query = format!("PRAGMA table_info('{}');", table_name);
    let mut stmt = conn.prepare(&query)?;

    let col_iter = stmt.query_map([], |row| {
        Ok(ColumnInfo {
            // Wir können über den Index zugreifen (PRAGMA liefert feste Spalten zurück)
            name: row.get(1)?,        // Spalte 'name'
            data_type: row.get(2)?,   // Spalte 'type'
            not_null: row.get(3)?,    // Spalte 'notnull' (wird automatisch zu bool)
            primary_key: row.get(5)?, // Spalte 'pk' (wird automatisch zu bool)
        })
    })?;

    let mut columns = Vec::new();
    for col in col_iter {
        columns.push(col?);
    }

    Ok(columns)
}
pub fn create_test_table(conn: &Connection) -> AnyResult<()> {
    conn.execute_batch(
        "-- 1. Fakultät
CREATE TABLE IF NOT EXISTS fakultaet (
    fakultaetsnummer INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    budget REAL NOT NULL CHECK(budget > 0)
);

-- 2. Studiengang
CREATE TABLE IF NOT EXISTS studiengang (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fakultaetsnummer INTEGER NOT NULL,
    name TEXT NOT NULL,
    abschluss TEXT NOT NULL CHECK(abschluss IN ('Bachelor', 'Master', 'Promotion')),
    
    -- Ein Studiengang gehört genau einer Fakultät
    FOREIGN KEY (fakultaetsnummer) REFERENCES fakultaet(fakultaetsnummer),
    
    -- Besitzt einen eindeutigen Namen INNERHALB seiner Fakultät
    UNIQUE (fakultaetsnummer, name)
);

-- 3. Student
CREATE TABLE IF NOT EXISTS student (
    matrikelnummer INTEGER PRIMARY KEY,
    vorname TEXT NOT NULL,
    nachname TEXT NOT NULL,
    geburtsdatum DATE NOT NULL CHECK(geburtsdatum <= date('now', '-16 years')),
    studiengang_id INTEGER NOT NULL,
    
    -- Beim Löschen eines Studiengangs dürfen Studenten nicht gelöscht werden.
    -- Da studiengang_id NOT NULL ist, müssen wir das Löschen des Studiengangs blockieren.
    FOREIGN KEY (studiengang_id) REFERENCES studiengang(id) ON DELETE RESTRICT
);

-- 4. Kurs
CREATE TABLE IF NOT EXISTS kurs (
    kurscode TEXT PRIMARY KEY,
    studiengang_id INTEGER NOT NULL,
    ects INTEGER NOT NULL CHECK(ects BETWEEN 1 AND 15),
    
    FOREIGN KEY (studiengang_id) REFERENCES studiengang(id)
);

-- 5. Belegung
CREATE TABLE IF NOT EXISTS belegung (
    matrikelnummer INTEGER NOT NULL,
    kurscode TEXT NOT NULL,
    semester TEXT NOT NULL, -- z.B. 'WS23', 'SS24'
    jahr INTEGER NOT NULL,
    note REAL CHECK(note BETWEEN 1.0 AND 5.0 OR note IS NULL),
    
    -- Beim Löschen eines Studenten sollen Belegungen gelöscht werden
    FOREIGN KEY (matrikelnummer) REFERENCES student(matrikelnummer) ON DELETE CASCADE,
    
    -- Beim Ändern eines Kurscodes sollen Belegungen aktualisiert werden
    FOREIGN KEY (kurscode) REFERENCES kurs(kurscode) ON UPDATE CASCADE,
    
    -- Ein Student darf denselben Kurs in einem Semester nur einmal belegen
    PRIMARY KEY (matrikelnummer, kurscode, semester)
);",
    )?;

    Ok(())
}
