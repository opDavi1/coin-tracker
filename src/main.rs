use std::error::Error;

mod database;

fn main() -> Result<(), Box<dyn Error>> {
    let connection = match database::init() {
        Ok(c) => c,
        Err(_) => {
            return Err(Box::from("Could not open the database"))
        }
    };
    Ok(())
}
