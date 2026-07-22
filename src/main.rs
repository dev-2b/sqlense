mod db;
fn main() {
    println!("Hello, world!");
    match db::establish_connection() {
        Ok(conn) => {
            // Use the database connection
            println!("Successfully established database connection.");
        }
        Err(e) => {
            eprintln!("Error establishing database connection: {}", e);
        }
    }
}
