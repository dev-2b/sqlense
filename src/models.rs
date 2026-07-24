//TableInfo ist hier unser DTO (Data Transfer Object)
#[derive(Debug)] // von Gemini als "best practice" empfohlen - werde noch herausfindenw arum
pub struct TableInfo {
    pub name: String, // NICHT &str, da wir die Daten aus der DB holen und sie dann in TableInfo speichern wollen. Wenn wir &str verwenden würden, wäre die Lebensdauer des Strings nicht garantiert.
    pub columns: Vec<ColumnInfo>, // Spalteninformationen für die Tabelle
}
#[derive(Debug)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub not_null: bool,
    pub primary_key: bool,
}
