use comfy_table::{modifiers::UTF8_ROUND_CORNERS, Cell, Table};
use crate::models::TableInfo;

pub fn print_tables(tables: &[TableInfo]) {
    for table_info in tables {
        println!("\n=== Tabelle: {} ===", table_info.name.to_uppercase());

        // Eine neue comfy-table Instanz erstellen
        let mut table = Table::new();
        
        // Ein schönes, abgerundetes Design anwenden
        table.apply_modifier(UTF8_ROUND_CORNERS);

        // Die Kopfzeile (Header) definieren
        table.set_header(vec![
            Cell::new("Spaltenname"),
            Cell::new("Datentyp"),
            Cell::new("Not Null"),
            Cell::new("Primary Key"),
        ]);

        // Über alle Spalten unseres Modells iterieren und als Zeilen einfügen
        for col in &table_info.columns {
            table.add_row(vec![
                Cell::new(&col.name),
                Cell::new(&col.data_type),
                // Wir wandeln den Boolean in ein schönes "Ja" oder "Nein" um
                Cell::new(if col.not_null { "Ja" } else { "Nein" }),
                Cell::new(if col.primary_key { "Ja" } else { "Nein" }),
            ]);
        }

        // Die fertige Tabelle ins Terminal drucken
        println!("{table}");
    }
}