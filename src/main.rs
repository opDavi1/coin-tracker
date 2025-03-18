// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

use std::error::Error;

use coin_tracker::database as db;

fn main() -> Result<(), Box<dyn Error>> {
    let connection = match db::init(None) {
        Ok(c) => c,
        Err(_) => {
            return Err(Box::from("Could not open the database"))
        }
    };
    Ok(())
}

