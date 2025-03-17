// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

use sqlite::{self, Connection, Error, State};

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

pub fn init() -> Result<Connection, Error> {
    let connection = sqlite::open("database.db")?;
    connection.execute(DATABASE_SQL)?;
    println!("Initialized the database");
    Ok(connection)
}

pub fn delete_coin(connection: &Connection, id: i64) -> Result<i64, Error> {
    let mut statement = connection.prepare("DELETE * FROM coins WHERE id = ?")?;
    statement.bind((1, id))?;
    match statement.next() {
        Ok(State::Done) => Ok(id),
        Ok(State::Row) => {
            return Err(Error {
                code: None, 
                message: Some("SQL Statement returned unexpected result".to_string())
            })
        },
        Err(e) => return Err(e),
        
    }
}

pub fn get_all_coins(connection: &Connection) -> Result<Vec<Coin>, Error> {
    let mut statement = connection.prepare("SELECT * FROM coins")?;
    let mut coins: Vec<Coin> = Vec::new();
    while let Ok(State::Row) = statement.next() {
        let coin = Coin::from_sql_row(&statement)?;
        coins.push(coin);
    }
    Ok(coins)
}

pub fn get_coin_by_id(connection: &Connection, id: &i64) -> Result<Coin, Error> {
    let mut statement = connection.prepare("SELECT * FROM coins WHERE id = ?")?;
    statement.bind((1, *id))?;
    match statement.next() {
        Ok(State::Row) => Coin::from_sql_row(&statement),
        Ok(State::Done) => {
            Err(Error {
                code: None,
                message: Some("SQL Statement returned unexpected result".to_string())
            })
        },
        Err(e) => return Err(e),
    }
}

pub fn get_coin_by_numista_id(connection: &Connection, numista_id: &i64) -> Result<Coin, Error> {
    let mut statement = connection.prepare("SELECT * FROM coins WHERE numista_id = ?")?;
    statement.bind((1, *numista_id))?;
    match statement.next() {
        Ok(State::Row) => Coin::from_sql_row(&statement),
        Ok(State::Done) => {
            Err(Error {
                code: None,
                message: Some("SQL Statement returned unexpected result".to_string())
            })
        },
        Err(e) => return Err(e),
    }
}
