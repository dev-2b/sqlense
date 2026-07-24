use anyhow::Result;

mod db;

fn main() -> Result<()> {
    // 1. Verbindung aufbauen. Wenn es fehlschlägt, bricht der ?-Operator
    // hier ab und anyhow gibt automatisch eine schöne Fehlermeldung im Terminal aus.
    let conn = db::establish_connection()?;
    println!("Successfully established database connection.");

    // 2. Tabellen erstellen. Auch hier: Bei Fehler bricht ? sicher ab.
    db::create_test_table(&conn)?;
    println!("Successfully initialized database schema.");

    // Wenn wir hier ankommen, lief alles fehlerfrei.
    Ok(())
}
