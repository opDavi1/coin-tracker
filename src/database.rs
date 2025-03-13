// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

use sqlite::{self, Connection};

use crate::coin::Coin;

const DATABASE_SQL: &str = "CREATE TABLE IF NOT EXISTS coins (
id INTEGER PRIMARY KEY AUTOINCREMENT,
numista_id INTEGER,
name TEXT NOT NULL,
coin_type INT,
issuer TEXT,
country TEXT,
min_year INT,
max_year INT,
composition TEXT,
shape INT,
diameter REAL,
thickness REAL,
weight REAL,
orientation INT,
denomination TEXT,
value REAL,
value_numerator INT,
value_denominator INT,
currency TEXT,
grade INT,
obverse_image TEXT,
reverse_image TEXT,
obverse_description TEXT,
reverse_description TEXT,
is_demonitized INT,
comments TEXT)";

pub fn init() -> Result<sqlite::Connection, sqlite::Error> {
    let connection = sqlite::open("database.db")?;
    connection.execute(DATABASE_SQL)?;
    println!("Initialized the database");
    Ok(connection)
}

fn delete_coin(connection: &Connection, coin: Coin) -> Result<i32, sqlite::Error> {
    todo!()
}
