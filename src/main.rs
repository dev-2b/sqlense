use anyhow::Result;

mod db;
mod models; // hier "bekannt geben" damit es mit use crate::models::TableInfo; in db.rs funktioniert
mod ui;

fn main() -> Result<()> {
    // 1. Verbindung aufbauen. Wenn es fehlschlägt, bricht der ?-Operator
    // hier ab und anyhow gibt automatisch eine schöne Fehlermeldung im Terminal aus.
    let conn = db::establish_connection()?;
    println!("Successfully established database connection.");

    // 2. Tabellen erstellen. Auch hier: Bei Fehler bricht ? sicher ab.
    db::create_test_table(&conn)?;
    println!("Successfully initialized database schema.");

    // 3. Tabellen auslesen und ausgeben
    let tables = db::get_tables(&conn)?;

    // Anzeige im Terminal. nur referenz übergeben, da wir die Daten nicht verändern wollen.
    ui::print_tables(&tables);

    // Wenn wir hier ankommen, lief alles fehlerfrei.
    Ok(())
}
