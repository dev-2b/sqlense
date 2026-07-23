mod db;
fn main() {
    println!("Hello, world!");
    match db::establish_connection() {
        Ok(conn) => {
            println!("Successfully established database connection.");
        }
        Err(e) => {
            eprintln!("Error establishing database connection: {}", e);
        }
    }
}
