use crate::models::TableInfo;
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
    let table_iter = stmt.query_map([], |row| {
        let table_name: String = row.get(0)?;

        // Hier bauen wir direkt unser DTO zusammen!
        Ok(TableInfo { name: table_name })
    })?;
    let mut tables = Vec::new();
    for table in table_iter {
        tables.push(table?);
    }

    Ok(tables)
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
