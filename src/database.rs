// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

use sqlite::{self, Connection, Error, State, Statement};

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

fn bind_coin_to_stmt(stmt: &mut Statement, coin: &Coin) -> Result<(), Error> {
    stmt.bind((1, coin.numista_id))?;
    stmt.bind((2, coin.name.as_str()))?;
    stmt.bind((3, coin.coin_type as i64))?;
    stmt.bind((4, coin.issuer.as_str()))?;
    stmt.bind((5, coin.country.as_str()))?;
    stmt.bind((6, coin.min_year))?;
    stmt.bind((7, coin.max_year))?;
    stmt.bind((8, coin.composition.as_str()))?;
    stmt.bind((9, coin.shape as i64))?;
    stmt.bind((10, coin.diameter))?;
    stmt.bind((11, coin.thickness))?;
    stmt.bind((12, coin.weight))?;
    stmt.bind((13, coin.orientation as i64))?;
    stmt.bind((14, coin.denomination.as_str()))?;
    stmt.bind((15, coin.value))?;
    stmt.bind((16, coin.value_numerator))?;
    stmt.bind((17, coin.value_denominator))?;
    stmt.bind((18, coin.currency.as_str()))?;
    stmt.bind((19, coin.grade as i64))?;
    stmt.bind((20, coin.obverse_image.as_str()))?;
    stmt.bind((21, coin.reverse_image.as_str()))?;
    stmt.bind((22, coin.obverse_description.as_str()))?;
    stmt.bind((23, coin.reverse_description.as_str()))?;
    stmt.bind((24, coin.is_demonitized as i64))?;
    stmt.bind((25, coin.comments.as_str()))?;
    Ok(())
}

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

pub fn insert_coin(connection: &Connection, coin: &Coin) -> Result<(), Error> {
    let mut statement = connection.prepare(
        "INSERT INTO coins ( \
        id, \
        numista_id \
        name, \
        coin_type, \
        issuer, \
        country, \
        min_year, \
        max_year, \
        composition, \
        shape, \
        diameter, \
        thickness, \
        weight, \
        orientation, \
        denomination, \
        value, \
        value_numerator, \
        value_denominator, \
        currency, \
        grade, \
        obverse_image, \
        reverse_image, \
        obverse_description, \
        reverse_description, \
        is_demonitized, \
        comments) \
        VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")?;
    bind_coin_to_stmt(&mut statement, &coin)?;
    match statement.next() {
        Ok(State::Done) => Ok(()),
        Ok(State::Row) => {
            Err(Error {
                code: None,
                message: Some("SQL Statement returned unexpected result".to_string())
            })
        },
        Err(e) => Err(e),
    }
}
