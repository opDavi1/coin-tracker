// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>
//
use std::error::Error;

mod coin;
mod database;

use coin::Coin;
use database as db;

fn main() -> Result<(), Box<dyn Error>> {
    let connection = match db::init() {
        Ok(c) => c,
        Err(_) => {
            return Err(Box::from("Could not open the database"))
        }
    };
    Ok(())
}

#[test]
fn database() {
    let connection = db::init().expect("Could not init db");
    let coin = Coin::new();
    let new_coin = Coin::example();
    let id = db::insert_coin(&connection, &coin).expect("Could not insert coin to db");
    db::update_coin(&connection, &id, &new_coin).expect("Could not update coin in db");
    let coin2 = db::get_coin_by_id(&connection, &id).expect("Could not get coin from db");
    println!("{:?}", coin2);
}
