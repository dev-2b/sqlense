# SQLENSE

Ein iterativ entwickeltes CLI- und TUI-Tool in Rust zur Untersuchung und Visualisierung von SQLite-Datenbanken. 

## 🎯 Projektziel
Dieses Projekt entsteht begleitend zu meiner Umschulung zum Fachinformatiker für Anwendungsentwicklung. Es dient dazu, theoretisches Wissen aus dem Schulunterricht (Datenbankdesign, SQL, DDL, DML) mit der Programmiersprache Rust in der Praxis zu vereinen. 

Der Fokus liegt auf sauberem Code, einem schrittweisen Architekturaufbau und dem Verständnis für relationale Datenbankstrukturen.

## 🚀 Roadmap

Das Projekt ist in mehrere aufeinander aufbauende Phasen unterteilt. Um die anfängliche Komplexität gering zu halten, startet das Tool als reines Kommandozeilenwerkzeug (CLI) und wird später um ein Text User Interface (TUI) erweitert.

### Phase 1: Fundament & Datenbankverbindung (CLI)
- [x] Setup des Rust-Projekts und Einbindung von `rusqlite`.
- [x] Aufbau einer sicheren Verbindung zu einer lokalen SQLite-Datenbankdatei.
- [x] Abfangen und sauberes Behandeln von Verbindungsfehlern (Error Handling in Rust).

### Phase 2: Der Schema-Visualisierer (Fokus: DDL)
- [x] Auslesen der Metadaten aus der Datenbank (Welche Tabellen existieren?).
- [x] Auslesen der Spaltenstruktur einer gewählten Tabelle (Datentypen, Primary Keys).
- [ ] Formatierte Ausgabe der Tabellenstrukturen im Terminal (z.B. mit dem Crate `comfy-table`).

### Phase 3: Datenanzeige & Basis-Interaktion (Fokus: DQL/DML)
- [ ] Implementierung eines einfachen `SELECT *`-Befehls zum Anzeigen von Tabelleninhalten.
- [ ] Argument-Parsing via CLI (z.B. `toolname show <tabelle>`), um gezielt Inhalte abzufragen.

### Phase 4: Such- und Filterfunktionen (Fokus: Erweitertes DML)
- [ ] Hinzufügen von Parametern für einfache Suchvorgänge.
- [ ] Generierung von `SELECT`-Statements mit `WHERE`-Klauseln basierend auf User-Input.
- [ ] Schutz vor SQL-Injection bei der Eingabe (Nutzung von Prepared Statements).

### Phase 5: Das TUI-Upgrade (Fokus: UI & State Management)
- [ ] Refactoring der bisherigen Codebasis zur sauberen Trennung von Datenbank-Logik und Anzeige.
- [ ] Einbindung des Crates `ratatui`.
- [ ] Bau einer interaktiven Terminal-Oberfläche zur Navigation durch Tabellen und Daten.

## Architektur und Projektstruktur

```text
sqlense/
├── Cargo.toml         # Verwaltung der Dependencies (Abhängigkeiten)
├── README.md          # Unsere Projektbeschreibung und Roadmap
├── test_db/           # Ordner für unsere lokale SQLite Test-Datenbank
│   └── dummy.db       
└── src/
    ├── main.rs        # Einstiegspunkt: Startet die App, delegiert Aufgaben
    ├── cli.rs         # Verarbeitung der Konsolenbefehle/Argumente
    ├── db.rs          # DAO-Schicht: Alles rund um `rusqlite` (Verbindung, Queries)
    ├── models.rs      # DTOs: Rust-Structs für unsere Datenbank-Entitäten
    └── ui.rs          # Ausgabe: Formatierte Konsolenausgabe (später Ratatui-TUI)