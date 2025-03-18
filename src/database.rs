// This file is a part of coin-tracker by opDavi1 licensed under the GPL-3.0-or-later license.
// See the included LICENSE.md file for more details or go to <https://www.gnu.org/licenses/>

use std::path::Path;

use rusqlite::{params, Connection, Result};
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

const INSERT_SQL: &str = "INSERT INTO coins  \
        VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

const UPDATE_SQL: &str = "UPDATE coins SET \
        numista_id = ?, \
        name = ?, \
        coin_type = ?, \
        issuer = ?, \
        country = ?, \
        min_year = ?, \
        max_year = ?, \
        composition = ?, \
        shape = ?, \
        diameter = ?, \
        thickness = ?, \
        weight = ?, \
        orientation = ?, \
        denomination = ?, \
        value = ?, \
        value_numerator = ?, \
        value_denominator = ?, \
        currency = ?, \
        grade = ?, \
        obverse_image = ?, \
        reverse_image = ?, \
        obverse_description = ?, \
        reverse_description = ?, \
        is_demonitized = ?, \
        comments = ? \
        WHERE id = ?";


pub fn init(file: Option<&Path>) -> Result<Connection> {
    let connection: Connection;
    match file {
        Some(f) => connection = Connection::open(f)?,
        None => connection = Connection::open("database.db")?,
    }
    connection.execute(DATABASE_SQL, ())?;
    println!("Initialized the database");
    Ok(connection)
}


pub fn delete_coin(connection: &Connection, id: &i64) -> Result<usize> {
    let mut statement = connection.prepare("DELETE * FROM coins WHERE id = ?")?;
    statement.execute([id])
}


pub fn get_all_coins(connection: &Connection) -> Result<Vec<Coin>> {
    let mut statement = connection.prepare("SELECT * FROM coins")?;
    let mut rows = statement.query([])?;
    let mut coins: Vec<Coin> = Vec::new();

    while let Some(row) = rows.next()? {
         coins.push(Coin::from_sql_row(row)?);
    }

    Ok(coins)
}


pub fn get_coin_by_id(connection: &Connection, id: &i64) -> Result<Coin> {
    let mut statement = connection.prepare("SELECT * FROM coins WHERE id = ?")?;
    statement.query_row([id], |row| {
        Coin::from_sql_row(row)
    })
}


pub fn get_coin_by_numista_id(connection: &Connection, numista_id: &i64) -> Result<Coin> {
    let mut statement = connection.prepare("SELECT * FROM coins WHERE numista_id = ?")?;
    statement.query_row([numista_id], |row| {
        Coin::from_sql_row(row)
    })
}


pub fn insert_coin(connection: &Connection, coin: &Coin) -> Result<i64> {
    let mut statement = connection.prepare(INSERT_SQL)?;
    let c = coin;
    statement.insert(params!(
        c.numista_id, 
        c.name, 
        c.coin_type as i64, 
        c.issuer, 
        c.country, 
        c.min_year, 
        c.max_year, 
        c.composition, 
        c.shape as i64, 
        c.diameter, 
        c.thickness, 
        c.weight,
        c.orientation as i64,
        c.denomination,
        c.value,
        c.value_numerator,
        c.value_denominator,
        c.currency,
        c.grade,
        c.obverse_image,
        c.reverse_image,
        c.obverse_description,
        c.reverse_description,
        c.is_demonitized,
        c.comments,
    ))
}


pub fn update_coin(connection: &Connection, id: &i64, new_coin: &Coin) -> Result<()> {
    let mut statement = connection.prepare(UPDATE_SQL)?;
    match statement.execute(params!(
        new_coin.numista_id, 
        new_coin.name, 
        new_coin.coin_type as i64, 
        new_coin.issuer, 
        new_coin.country, 
        new_coin.min_year, 
        new_coin.max_year, 
        new_coin.composition, 
        new_coin.shape as i64, 
        new_coin.diameter, 
        new_coin.thickness, 
        new_coin.weight,
        new_coin.orientation as i64,
        new_coin.denomination,
        new_coin.value,
        new_coin.value_numerator,
        new_coin.value_denominator,
        new_coin.currency,
        new_coin.grade,
        new_coin.obverse_image,
        new_coin.reverse_image,
        new_coin.obverse_description,
        new_coin.reverse_description,
        new_coin.is_demonitized,
        new_coin.comments,
        id, 
    )) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
