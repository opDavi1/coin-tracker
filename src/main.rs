// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>
//
use std::error::Error;

mod coin;
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
