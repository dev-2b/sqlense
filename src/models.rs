#[derive(Debug)] // von Gemini als "best practice" empfohlen - werde noch herausfindenw arum
pub struct TableInfo {
    pub name: String, // NICHT &str, da wir die Daten aus der DB holen und sie dann in TableInfo speichern wollen. Wenn wir &str verwenden würden, wäre die Lebensdauer des Strings nicht garantiert.
}
